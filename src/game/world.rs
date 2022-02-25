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

const ACTORS_BROADCAST_CYCLES: u8 = 3;

pub struct World {
    pub actors: Vec<Box<dyn GameItem>>,
    pub offset: Coordinates,
    pub ui: Vec<Box<dyn GameItem>>,
    spawner: Spawner,

    actors_viewport: Option<Viewport>,
    ui_viewport: Option<Viewport>,
    world_viewport: Viewport,
}

impl Default for World {
    fn default() -> Self {
        Self {
            actors: Vec::new(),
            offset: Coordinates::default(),
            spawner: Spawner::default(),
            ui: Vec::new(),

            actors_viewport: None,
            ui_viewport: None,
            world_viewport: Viewport::new_for_world(),
        }
    }
}

impl World {
    pub fn broadcast_collisions(&mut self) -> Result<()> {
        let commands = self.detect_collisions();
        self.broadcast_commands(&commands)
    }

    pub fn broadcast_commands(&mut self, commands: &[Command]) -> Result<()> {
        let commands: Vec<Command> = commands
            .iter()
            .flat_map(|command| self.broadcast_command(*command))
            .collect();
        validate_unhandled_commands(&commands)
    }

    pub fn broadcast_actors_viewport(&mut self, viewport: Viewport) -> Result<()> {
        if let Some(previous_viewport) = self.actors_viewport {
            if previous_viewport != viewport {
                self.broadcast_commands(&[Command::ActorsViewportChanged(viewport)])?;
            }
        } else {
            self.broadcast_commands(&[Command::ActorsViewportInitialized(viewport)])?;
        }
        self.actors_viewport = Some(viewport);
        Ok(())
    }

    pub fn broadcast_ui_viewport(&mut self, viewport: Viewport) -> Result<()> {
        if let Some(previous_viewport) = self.ui_viewport {
            if previous_viewport != viewport {
                self.broadcast_command_to_ui(Command::UiViewportInitializedOrChanged(viewport))?;
            }
        } else {
            self.broadcast_command_to_ui(Command::UiViewportInitializedOrChanged(viewport))?;
        }
        self.ui_viewport = Some(viewport);
        Ok(())
    }

    pub fn handle_tick(&mut self, ticker: &Ticker) {
        // `tick.number` 1 is the fist time `handle_tick()` is invoked, because it is invoked *after* each tick,
        // which can occur upon initial startup or after a "Restart" command.
        if ticker.number() == 1 {
            // Clear the Viewports so that ViewportInitialized commands will be broadcast,
            // which is necessary to re-align the Ship and UI.
            self.actors_viewport = None;
            self.ui_viewport = None;

            // Actors are updated on every tick, but the UI is initialized only once on the first tick.
            self.ui = self.spawner.ui();
        }

        let world_viewport = self.world_viewport;
        self.game_items_iter_mut()
            .for_each(|handler| handler.handle_tick(ticker, world_viewport));
        self.actors.retain(|actor| !actor.deleted());
        self.actors
            .extend(self.spawner.actors(ticker, world_viewport));
    }

    fn broadcast_command(&mut self, command: Command) -> Vec<Command> {
        let mut commands = vec![command];
        for _ in 0..ACTORS_BROADCAST_CYCLES {
            commands = commands
                .into_iter()
                .flat_map(|command| self.notify_handlers(command))
                .collect();
        }
        commands
    }

    fn broadcast_command_to_ui(&mut self, command: Command) -> Result<()> {
        let commands = self.notify_ui_handlers(command);
        validate_unhandled_commands(&commands)
    }

    fn consumed_command(&mut self, command: Command) -> bool {
        match command {
            Command::ActorsViewportChanged(viewport) => {
                self.align_offset(viewport);
                return false;
            }
            Command::AddBullet(coordinates) => self.actors.push(Box::new(Bullet::new(coordinates))),
            Command::AddExplosion(coordinates) => {
                self.actors.push(Box::new(Explosion::new(coordinates)))
            }
            Command::AddMissile(coordinates) => {
                self.actors.push(Box::new(Missile::new(coordinates)))
            }
            Command::MoveOffset(movement) => self.offset.movement(movement),
            Command::Restart => {
                self.actors.clear();
                self.offset = Coordinates::default();
                self.spawner.restart();
                return false;
            }
            _ => return false,
        }
        true
    }

    fn align_offset(&mut self, viewport: Viewport) {
        let (x_previous, _) = self.actors_viewport.unwrap().centered().as_tuple();
        let (x_current, _) = viewport.centered().as_tuple();
        let dx = i16::from(x_current) - i16::from(x_previous);
        self.offset.offset_x(dx);
    }

    fn detect_collisions(&mut self) -> Vec<Command> {
        let mut commands = Vec::new();
        for index in 0..self.actors.len().saturating_sub(1) {
            let (left_actors, right_actors) = self.actors.split_at_mut(index + 1);
            let left_actor = &mut left_actors[index];
            if left_actor.deleted() {
                // Do not detect collisions on deleted actors,
                // which can occur when the ship moves faster than once per tick.
                continue;
            }
            for right_actor in right_actors {
                if right_actor.deleted() {
                    continue;
                }
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
            .flat_map(move |handler| handler.handle_command(command))
            .collect::<Vec<Command>>()
    }

    fn notify_ui_handlers(&mut self, command: Command) -> Vec<Command> {
        self.ui
            .iter_mut()
            .flat_map(move |handler| handler.handle_command(command))
            .collect::<Vec<Command>>()
    }
}

fn validate_unhandled_commands(commands: &[Command]) -> Result<()> {
    if commands.is_empty() {
        Ok(())
    } else {
        Err(anyhow!(
            "Error: There are unhandled command(s): {:?}",
            commands
        ))
    }
}
