use tui::widgets::canvas::Context;

use crate::{
    clock::{
        countdown::Countdown,
        ticker::{TickHandler, Ticker},
    },
    command::{Command, CommandHandler},
    game::game_item::{GameItem, GameItemKind},
    view::{
        render::{render_text, Renderable},
        viewport::{Coordinates, Viewport},
    },
};

use super::button_kind::ButtonKind;

const ACTIVE_COUNT: u16 = 8;

#[derive(Clone, Debug)]
pub struct Button {
    active: Countdown,
    button_kind: ButtonKind,
    coordinates: Coordinates,
    disabled: Countdown,
}

impl CommandHandler for Button {
    fn handle_command(&mut self, command: Command) -> Command {
        match (&self.button_kind, command) {
            (ButtonKind::Missile, Command::FireMissile) => self.update_counters(),
            (ButtonKind::Rewind, Command::Rewind) => self.update_counters(),
            (ButtonKind::Shields, Command::ActivateShields) => self.update_counters(),
            (_, _) => (),
        }
        Command::NOOP
    }
}

impl GameItem for Button {
    fn kind(&self) -> GameItemKind {
        GameItemKind::Button
    }
}

impl Renderable for Button {
    fn render(&mut self, context: &mut Context, _: Viewport) {
        render_text(
            context,
            self.coordinates,
            self.button_kind.text(),
            self.button_kind.color(self.active.on(), self.disabled.on()),
        );
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_from_coordinates(self.width(), self.height(), self.coordinates)
    }
}

impl TickHandler for Button {
    fn handle_tick(&mut self, _: &Ticker) {
        self.active.down();
        self.disabled.down();
    }
}

impl Button {
    pub fn new_game_over() -> Self {
        Self::new(ButtonKind::GameOver)
    }

    pub fn new_missile() -> Self {
        Self::new(ButtonKind::Missile)
    }

    pub fn new_rewind() -> Self {
        Self::new(ButtonKind::Rewind)
    }

    pub fn new_shields() -> Self {
        Self::new(ButtonKind::Shields)
    }

    pub fn height(&self) -> u16 {
        self.button_kind.text().lines().count() as u16
    }

    pub fn set_coordinates(&mut self, coordinates: Coordinates) {
        self.coordinates = coordinates;
    }

    pub fn width(&self) -> u16 {
        self.button_kind
            .text()
            .lines()
            .next()
            .unwrap()
            .chars()
            .count() as u16
    }

    fn new(button_kind: ButtonKind) -> Self {
        Self {
            active: Countdown::new(ACTIVE_COUNT),
            button_kind,
            coordinates: (0, 0), // `ButtonPanel` will update the coordinates before rendering.
            disabled: Countdown::new(button_kind.disabled_count()),
        }
    }

    fn update_counters(&mut self) {
        self.active.restart();
        if self.disabled.off() {
            self.disabled.restart();
        }
    }
}
