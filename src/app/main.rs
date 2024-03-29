use super::{
    command::Command,
    input::{receive_input_commands, send_input_commands},
};
use crate::{
    clock::ticker::Ticker,
    game::world::World,
    view::{render::render, session::Session},
};
use anyhow::Result;

use std::{sync::mpsc, thread, time::Duration};

const MAIN_LOOP_MIN_PERIOD_MS: u64 = 20;
const TICK_PERIOD_MS: u64 = 100;
pub const TICKS_PER_SECOND: u16 = 1000 / TICK_PERIOD_MS as u16;

pub struct App {
    ticker: Ticker,
    world: World,
}

impl Default for App {
    fn default() -> Self {
        Self {
            ticker: Ticker::new(Duration::from_millis(TICK_PERIOD_MS)),
            world: World::default(),
        }
    }
}

impl App {
    pub fn run(&mut self, session: &mut Session) -> Result<()> {
        let min_period = Duration::from_millis(MAIN_LOOP_MIN_PERIOD_MS);
        let (tx, rx) = mpsc::channel();
        send_input_commands(tx);

        loop {
            let ticked = self.ticker.maybe_tick();
            if ticked {
                self.world.handle_tick(&self.ticker);
            }

            let commands = receive_input_commands(&rx);
            for command in commands.iter() {
                match command {
                    Command::Quit => return Ok(()),
                    Command::Restart => self.ticker.restart(),
                    _ => (),
                }
            }

            if ticked || !commands.is_empty() {
                self.world.broadcast_commands(&commands)?;
                render(session, &mut self.world)?;

                // A tick, command, or render can cause a collision.
                self.world.broadcast_collisions()?;
            }
            thread::sleep(min_period);
        }
    }
}
