use super::button_kind::ButtonKind;
use crate::{
    app::command::{Command, CommandHandler},
    clock::{
        countdown::Countdown,
        ticker::{TickHandler, Ticker},
    },
    game::game_item::GameItem,
    view::{
        render::{render_text, Renderable},
        util::{chars_height, chars_width},
        viewport::{Coordinates, Viewport},
    },
};
use tui::widgets::canvas::Context;

const ACTIVE_COUNT: u16 = 8;

#[derive(Copy, Clone)]
pub enum ButtonSize {
    Condensed,
    Full,
}

pub struct Button {
    active: Countdown,
    button_kind: ButtonKind,
    button_size: ButtonSize,
    coordinates: Coordinates,
    disabled: Countdown,
}

impl CommandHandler for Button {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        let mut commands = Vec::new();
        match (&self.button_kind, command) {
            (ButtonKind::Missile, Command::PressMissileButton) => {
                self.maybe_fire(&mut commands, Command::FireMissile)
            }
            (ButtonKind::Rewind, Command::PressRewindButton) => {
                self.maybe_fire(&mut commands, Command::FireRewind)
            }
            (ButtonKind::Shields, Command::PressShieldsButton) => {
                self.maybe_fire(&mut commands, Command::FireShields)
            }
            (_, _) => (),
        }
        commands
    }
}

impl GameItem for Button {}

impl Renderable for Button {
    fn render(&mut self, context: &mut Context, viewport: &Viewport) {
        self.button_size = if viewport.rect.width < 68 {
            ButtonSize::Condensed
        } else {
            ButtonSize::Full
        };
        render_text(
            context,
            self.coordinates,
            self.button_kind.text(self.button_size),
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
        chars_height(self.button_kind.text(self.button_size))
    }

    pub fn set_coordinates(&mut self, coordinates: Coordinates) {
        self.coordinates = coordinates;
    }

    pub fn width(&self) -> u16 {
        chars_width(self.button_kind.text(self.button_size))
    }

    fn new(button_kind: ButtonKind) -> Self {
        let disabled_count = button_kind.disabled_count();
        Self {
            active: Countdown::new(ACTIVE_COUNT),
            button_kind,
            button_size: ButtonSize::Full,
            coordinates: (0, 0), // `ButtonPanel` will update the coordinates before rendering.
            disabled: Countdown::new(disabled_count),
        }
    }

    fn maybe_fire(&mut self, commands: &mut Vec<Command>, command: Command) {
        self.active.restart();
        if self.disabled.off() {
            self.disabled.restart();
            commands.push(command);
        }
    }
}
