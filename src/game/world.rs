use crate::{
    clock::ticker::{TickHandler, Ticker},
    command::Command,
    view::viewport::Viewport,
};

use super::{
    actors::{bullet::Bullet, explosion::Explosion, missile::Missile},
    spawner::Spawner,
    GameItem,
};

pub struct World {
    pub actors: Vec<Box<dyn GameItem>>,
    pub buttons: Vec<Box<dyn GameItem>>,
    spawner: Spawner,
    viewport: Option<Viewport>,
}

impl Default for World {
    fn default() -> Self {
        let mut obj = Self {
            actors: Vec::new(),
            buttons: Vec::new(),
            spawner: Spawner::default(),
            viewport: None,
        };
        obj.restart();
        obj
    }
}

impl TickHandler for World {
    fn handle_tick(&mut self, ticker: &Ticker) {
        self.game_items_iter_mut()
            .for_each(|handler| handler.handle_tick(ticker));
        self.actors.retain(|actor| !actor.deleted());
        self.actors.extend(self.spawner.actors(ticker));
        self.spawner.next(); // TODO transition levels.
    }
}

impl World {
    pub fn set_viewport(&mut self, viewport: Viewport) {
        self.viewport = Some(viewport);
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
                    commands.push(left_actor.handle_command(Command::Collide(right_actor.kind())));
                    commands.push(right_actor.handle_command(Command::Collide(left_actor.kind())));
                }
            }
        }
        commands.retain(|command| *command != Command::NOOP);
        commands
    }

    pub fn broadcast_commands(&mut self, commands: Vec<Command>) -> Vec<Command> {
        if contains_quit_command(&commands) {
            return vec![Command::Quit];
        }
        commands
            .into_iter()
            .flat_map(|command| self.broadcast_command(command))
            .filter(|command| *command != Command::NOOP)
            .collect()
    }

    fn broadcast_command(&mut self, command: Command) -> Vec<Command> {
        let commands = self.notify_handlers(command);
        if contains_quit_command(&commands) {
            return vec![Command::Quit];
        }
        commands
            .into_iter()
            .flat_map(|command| self.notify_handlers(command))
            .collect()
    }

    fn game_items_iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn GameItem>> {
        self.actors.iter_mut().chain(self.buttons.iter_mut())
    }

    fn handle_command(&mut self, command: Command) {
        match command {
            Command::AddBullet(coordinates) => self.actors.push(Box::new(Bullet::new(coordinates))),
            Command::AddExplosion(coordinates) => {
                self.actors.push(Box::new(Explosion::new(coordinates)))
            }
            Command::AddMissile(coordinates) => {
                self.actors.push(Box::new(Missile::new(coordinates)))
            }
            Command::Restart => self.restart(),
            _ => (),
        }
    }

    fn notify_handlers(&mut self, command: Command) -> Vec<Command> {
        self.handle_command(command);
        self.game_items_iter_mut()
            .map(|handler| handler.handle_command(command))
            .collect()
    }

    fn restart(&mut self) {
        self.spawner.restart();
        self.actors.clear(); // Actors are created in `handle_tick`.
        self.buttons = self.spawner.buttons();
    }
}

fn contains_quit_command(commands: &Vec<Command>) -> bool {
    commands.iter().any(|command| *command == Command::Quit)
}
