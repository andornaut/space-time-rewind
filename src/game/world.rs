use super::{
    actors::{bullet::Bullet, explosion::Explosion, missile::Missile},
    game_item::GameItem,
    spawner::spawner::Spawner,
};
use crate::{
    app::command::{Command, NO_COMMANDS},
    clock::ticker::{TickHandler, Ticker},
    view::viewport::Viewport,
};
use anyhow::{anyhow, Result};

pub struct World {
    pub actors: Vec<Box<dyn GameItem>>,
    pub ui: Vec<Box<dyn GameItem>>,
    actors_viewport: Option<Viewport>,
    spawner: Spawner,
}

impl Default for World {
    fn default() -> Self {
        Self {
            actors: Vec::new(),
            actors_viewport: None,
            spawner: Spawner::default(),
            ui: Vec::new(),
        }
    }
}

impl TickHandler for World {
    fn handle_tick(&mut self, ticker: &Ticker) {
        if ticker.first() {
            self.restart();
            return;
        }

        self.game_items_iter_mut()
            .for_each(|handler| handler.handle_tick(ticker));
        self.actors.retain(|actor| !actor.deleted());

        let viewport = &self.actors_viewport.unwrap();
        self.actors.extend(self.spawner.actors(ticker, viewport));
    }
}

impl World {
    pub fn broadcast_commands(&mut self, commands: Vec<Command>) -> Result<Command> {
        if contains_quit_commands(&commands) {
            return Ok(Command::Quit);
        }
        let commands: Vec<Command> = commands
            .into_iter()
            .flat_map(|command| self.broadcast_command(command))
            .collect();
        let commands = commands
            .into_iter()
            .filter(|command| !self.consumed_command(*command))
            .collect();

        if contains_unhandled_commands(&commands) {
            return Err(anyhow!(
                "Error: There are unhandled command(s): {:?}",
                commands
            ));
        }
        Ok(if contains_quit_commands(&commands) {
            Command::Quit
        } else {
            Command::Continue
        })
    }

    pub fn detect_collisions(&mut self) -> Vec<Command> {
        let mut commands = Vec::new();
        let len = self.actors.len();
        if len == 0 {
            return commands;
        }
        for index in 0..len - 1 {
            let (left_actors, right_actors) = self.actors.split_at_mut(index + 1);
            let left_actor = &mut left_actors[index];
            for right_actor in right_actors {
                if left_actor.viewport().intersects(&right_actor.viewport()) {
                    commands
                        .extend(left_actor.handle_command(Command::Collide(right_actor.kind())));
                    commands
                        .extend(right_actor.handle_command(Command::Collide(left_actor.kind())));
                }
            }
        }
        commands
    }

    pub fn set_actors_viewport(&mut self, viewport: Viewport) {
        self.actors_viewport = Some(viewport);
    }

    fn broadcast_command(&mut self, command: Command) -> Vec<Command> {
        let secondary_commands = self.notify_handlers(command);
        if contains_quit_commands(&secondary_commands) {
            return vec![Command::Quit];
        }
        secondary_commands
            .into_iter()
            .flat_map(|command| self.notify_handlers(command))
            .collect()
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
            Command::Restart => self.restart(),
            _ => return false,
        }
        true
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

    fn restart(&mut self) {
        self.spawner.restart();
        self.actors.clear(); // Actors are created in `handle_tick`.
        self.ui = self.spawner.ui();
    }
}

fn contains_quit_commands(commands: &Vec<Command>) -> bool {
    commands.iter().any(|command| *command == Command::Quit)
}

fn contains_unhandled_commands(commands: &Vec<Command>) -> bool {
    commands.iter().any(|command| *command != Command::Quit)
}
