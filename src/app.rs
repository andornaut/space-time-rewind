use crate::{
    clock::ticker::{TickHandler, Ticker},
    command::Command,
    game::world::World,
    session::Session,
    view::{
        factory::{
            create_actors_block, create_actors_viewport, create_background_block,
            create_buttons_block, create_buttons_viewport, split_into_actors_and_buttons,
        },
        render::render_canvas,
    },
};
use anyhow::{Error, Result};
use crossterm::event::{poll, read, Event};
use std::time::{Duration, Instant};

const TICK_RATE_MS: u64 = 20;

pub struct App {
    last_tick: Option<Instant>,
    tick_rate: Duration,
    ticker: Ticker,
    world: World,
}

impl Default for App {
    fn default() -> Self {
        Self {
            last_tick: None,
            tick_rate: Duration::from_millis(TICK_RATE_MS),
            ticker: Ticker::default(),
            world: World::default(),
        }
    }
}

impl App {
    pub fn run(&mut self, session: &mut Session) -> Result<()> {
        loop {
            self.maybe_tick();
            self.render(session)?;

            for command in self.process_commands()? {
                match command {
                    Command::Quit => return Ok(()),
                    _ => panic!("`World.broadcast_commands` must return an empty Vector or one containing a single item: Command::Quit"),
                }
            }
        }
    }

    fn process_commands(&mut self) -> Result<Vec<Command>, Error> {
        let mut commands = self.world.detect_collisions();
        let command = self.wait_for_input_command()?;
        match command {
            Some(command) => commands.push(command),
            None => (),
        }
        Ok(self.world.broadcast_commands(commands))
    }

    fn maybe_tick(&mut self) {
        let last_tick = match self.last_tick {
            Some(tick) => tick,
            None => self.reset_last_tick(),
        };
        if last_tick.elapsed() >= self.tick_rate {
            self.reset_last_tick();
            self.ticker.tick();
            self.world.handle_tick(&self.ticker);
        }
    }

    fn remaining_timeout(&self) -> Duration {
        let elapsed = self.last_tick.unwrap_or_else(|| Instant::now()).elapsed();
        self.tick_rate
            .checked_sub(elapsed)
            .unwrap_or_else(|| Duration::from_secs(0))
    }

    fn render<'a>(&'a mut self, session: &'a mut Session) -> Result<(), Error> {
        session.terminal.draw(|frame| {
            let window = frame.size();
            // Set the background color of the entire terminal window.
            frame.render_widget(create_background_block(), window);

            let (actors_rect, buttons_rect) = split_into_actors_and_buttons(window);
            let actors_viewport = create_actors_viewport(actors_rect);
            let buttons_viewport = create_buttons_viewport(actors_rect);
            self.world.set_viewport(actors_viewport);
            render_canvas(
                frame,
                &mut self.world.actors,
                create_actors_block(),
                actors_rect,
                actors_viewport,
            );
            render_canvas(
                frame,
                &mut self.world.buttons,
                create_buttons_block(),
                buttons_rect,
                buttons_viewport,
            );
        })?;
        Ok(())
    }

    fn reset_last_tick(&mut self) -> Instant {
        let now = Instant::now();
        self.last_tick = Some(now);
        now
    }

    fn wait_for_input_command(&mut self) -> Result<Option<Command>, Error> {
        if poll(self.remaining_timeout())? {
            // `poll()` returned true, so an event is available,
            // so the following call to `read()` will not block.
            if let Event::Key(key) = read()? {
                return Ok(Some(Command::from(key)));
            }
        }
        Ok(None)
    }
}
