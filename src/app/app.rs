use super::command::Command;
use crate::{
    clock::ticker::{TickHandler, Ticker},
    game::world::World,
    view::{render::render, session::Session},
};
use anyhow::Result;
use crossterm::event::{poll, read, Event};
use std::time::Duration;

const TICK_RATE_MS: u64 = 20;
pub const TICKS_PER_SECOND: u16 = 1000 / TICK_RATE_MS as u16;

pub struct App {
    ticker: Ticker,
    world: World,
}

impl Default for App {
    fn default() -> Self {
        Self {
            ticker: Ticker::new(Duration::from_millis(TICK_RATE_MS)),
            world: World::default(),
        }
    }
}
impl App {
    pub fn run(&mut self, session: &mut Session) -> Result<()> {
        loop {
            if self.ticker.maybe_tick() {
                self.world.handle_tick(&self.ticker);
            }
            render(session, &mut self.world)?;

            let mut commands = self.world.detect_collisions();

            match self.wait_for_input_command()? {
                Some(command) => match command {
                    Command::Quit => return Ok(()),
                    Command::Restart => self.ticker.restart(),
                    _ => commands.push(command),
                },
                None => (),
            }
            self.world.broadcast_commands(commands)?;
        }
    }

    fn wait_for_input_command(&mut self) -> Result<Option<Command>> {
        if poll(self.ticker.remaining_timeout())? {
            // `poll()` returned true, so an event is available,
            // so the following call to `read()` will not block.
            if let Event::Key(key) = read()? {
                return Ok(Some(Command::from(key)));
            }
        }
        Ok(None)
    }
}
