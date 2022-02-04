use super::{
    command::Command,
    input::{receive_input_commands, send_input_commands},
};
use crate::{
    clock::ticker::{TickHandler, Ticker},
    game::world::World,
    view::{render::render, session::Session},
};
use anyhow::Result;

use std::{sync::mpsc, thread, time::Duration};

const TICK_RATE_MS: u64 = 40;
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
        let (tx, rx) = mpsc::channel();
        send_input_commands(tx);

        loop {
            if self.ticker.maybe_tick() {
                self.world.handle_tick(&self.ticker);
            }
            render(session, &mut self.world)?;

            let mut commands = receive_input_commands(&rx);
            for command in commands.iter() {
                match command {
                    Command::Quit => return Ok(()),
                    Command::Restart => self.ticker.restart(),
                    _ => (),
                }
            }
            commands.extend(self.world.detect_collisions());
            self.world.broadcast_commands(commands)?;
            thread::sleep(self.ticker.remaining_timeout());
        }
    }
}
