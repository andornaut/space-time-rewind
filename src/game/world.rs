use super::{
    actors::{bullet::Bullet, explosion::Explosion, missile::Missile},
    game_item::GameItem,
    spawner::spawner::Spawner,
};
use crate::{
    app::command::{Command, NO_COMMANDS},
    clock::ticker::Ticker,
    view::{coordinates::Coordinates, viewport::Viewport},
};
use anyhow::{anyhow, Result};

const BROADCAST_CYCLES: u8 = 2;

pub struct World {
    pub actors: Vec<Box<dyn GameItem>>,
    pub offset: Coordinates,
    pub ui: Vec<Box<dyn GameItem>>,
    spawner: Spawner,
    viewport: Viewport,
}

impl Default for World {
    fn default() -> Self {
        Self {
            actors: Vec::new(),
            offset: Coordinates::default(),
            spawner: Spawner::default(),
            ui: Vec::new(),
            viewport: Viewport::new_for_world(),
        }
    }
}

impl World {
    pub fn broadcast_collisions(&mut self) -> Result<()> {
        let commands = self.detect_collisions();
        if !commands.is_empty() {
            self.broadcast_commands(commands)?;
        }
        Ok(())
    }

    pub fn broadcast_commands(&mut self, commands: Vec<Command>) -> Result<()> {
        let commands: Vec<Command> = commands
            .into_iter()
            .flat_map(|command| self.broadcast_command(command))
            .collect();
        let unhandled_commands: Vec<Command> = commands
            .into_iter()
            .filter(|command| !self.consumed_command(*command))
            .collect();

        if !unhandled_commands.is_empty() {
            return Err(anyhow!(
                "Error: There are unhandled command(s): {:?}",
                unhandled_commands
            ));
        }
        Ok(())
    }

    pub fn handle_tick(&mut self, ticker: &Ticker) {
        // `tick.number` 1 is the fist time `handle_tick()` is invoked, because it is invoked *after* each tick.
        if ticker.number() == 1 {
            // This can occur upon initial startup and also after a "Restart" command.
            self.spawner.restart();
            self.actors.clear();
            self.offset = Coordinates::default();
            self.ui = self.spawner.ui();
        }

        let viewport = self.viewport;
        self.game_items_iter_mut()
            .for_each(|handler| handler.handle_tick(ticker, &viewport));
        self.actors.retain(|actor| !actor.deleted());
        self.actors.extend(self.spawner.actors(ticker, &viewport));
    }

    fn broadcast_command(&mut self, command: Command) -> Vec<Command> {
        let mut commands = self.notify_handlers(command);
        for _ in 0..BROADCAST_CYCLES {
            commands = commands
                .into_iter()
                .flat_map(|command| self.notify_handlers(command))
                .collect();
        }
        commands
    }

    fn consumed_command(&mut self, command: Command) -> bool {
        match command {
            Command::AddBullet(coordinates) => self.actors.push(Box::new(Bullet::new(coordinates))),
            Command::AddExplosion(coordinates) => {
                self.actors.push(Box::new(Explosion::new(coordinates)))
            }
            Command::AddMissile(coordinates) => {
                self.actors.push(Box::new(Missile::new(coordinates)))
            }
            Command::MoveShip((dx, _)) => {
                // Offset the ship's horizontal movement.
                self.offset.movement((-dx, 0));
                return false;
            }
            _ => return false,
        }
        true
    }

    fn detect_collisions(&mut self) -> Vec<Command> {
        let mut commands = Vec::new();
        for index in 0..self.actors.len().saturating_sub(1) {
            let (left_actors, right_actors) = self.actors.split_at_mut(index + 1);
            let left_actor = &mut left_actors[index];
            for right_actor in right_actors {
                if left_actor.viewport().intersects(right_actor.viewport()) {
                    commands
                        .extend(left_actor.handle_command(Command::Collide(right_actor.kind())));
                    commands
                        .extend(right_actor.handle_command(Command::Collide(left_actor.kind())));
                }
            }
        }
        commands
    }

    fn game_items_iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn GameItem>> {
        self.actors.iter_mut().chain(self.ui.iter_mut())
    }

    fn notify_handlers(&mut self, command: Command) -> Vec<Command> {
        if self.consumed_command(command) {
            return NO_COMMANDS;
        }
        self.game_items_iter_mut()
            .flat_map(|handler| handler.handle_command(command))
            .collect()
    }
}
