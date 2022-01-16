use anyhow::{Error, Result};
use crossterm::event::{poll, read, Event};
use std::time::{Duration, Instant};

use crate::{
    clock::ticker::{TickHandler, Ticker},
    command::Command,
    game::world::World,
    session::Session,
    view::{
        render::render_canvas,
        viewport_factory::{
            create_actors_viewport, create_buttons_viewport, split_into_actors_and_buttons,
        },
        widget_factory::{create_actors_block, create_background_block, create_buttons_block},
    },
};

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

            // `poll` blocks until timeout elapses or a keyboard event is received.
            if poll(self.remaining_timeout())? {
                // `poll()` returned true, so an event is available,
                // so the following call to `read()` will not block.
                if let Event::Key(key) = read()? {
                    commands.push(Command::from(key));
                }
            }

            for command in self.world.broadcast_commands(commands) {
                match command {
                    Command::Quit => return Ok(()),
                    _ => panic!("`World.broadcast_commands` must return an empty Vector or one containing a single item: Command::Quit"),
                }
            }
        }
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

    fn render<'a>(&'a mut self, session: &'a mut Session) -> Result<(), Error> {
        session.terminal.draw(|frame| {
            let (actors_rect, buttons_rect) = split_into_actors_and_buttons(frame.size());
            let actors_viewport = create_actors_viewport(actors_rect);
            let buttons_viewport = create_buttons_viewport(actors_rect);
            self.world.set_viewport(actors_viewport);

            frame.render_widget(create_background_block(), frame.size());
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

    fn remaining_timeout(&self) -> Duration {
        let elapsed = self.last_tick.unwrap_or_else(|| Instant::now()).elapsed();
        self.tick_rate
            .checked_sub(elapsed)
            .unwrap_or_else(|| Duration::from_secs(0))
    }

    fn reset_last_tick(&mut self) -> Instant {
        let now = Instant::now();
        self.last_tick = Some(now);
        now
    }
}
