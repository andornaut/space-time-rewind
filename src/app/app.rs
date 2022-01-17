use super::command::Command;
use crate::{
    clock::ticker::{TickHandler, Ticker},
    game::world::World,
    view::{
        factory::{
            create_actors_block, create_actors_viewport, create_background_block,
            create_buttons_block, create_buttons_viewport, split_into_actors_and_buttons,
        },
        render::render_canvas,
        session::Session,
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

            let mut commands = self.world.detect_collisions();
            self.maybe_add_input_command(&mut commands)?;
            if let Command::Quit = self.world.broadcast_commands(commands)? {
                return Ok(());
            }
        }
    }

    fn maybe_add_input_command(&mut self, commands: &mut Vec<Command>) -> Result<(), Error> {
        match self.wait_for_input_command()? {
            Some(command) => commands.push(command),
            None => (),
        }
        Ok(())
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

    fn render(&mut self, session: &mut Session) -> Result<()> {
        session.terminal.draw(|frame| {
            let window = frame.size();
            // Set the background color of the entire terminal window.
            frame.render_widget(create_background_block(), window);

            let (actors_rect, buttons_rect) = split_into_actors_and_buttons(window);
            let actors_viewport = create_actors_viewport(actors_rect);
            let buttons_viewport = create_buttons_viewport(actors_rect);
            self.world.set_actors_viewport(actors_viewport);
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

    fn wait_for_input_command(&mut self) -> Result<Option<Command>> {
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
