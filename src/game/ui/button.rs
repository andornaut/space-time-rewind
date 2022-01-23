use super::button_kind::ButtonKind;
use crate::{
    app::{
        app::TICKS_PER_SECOND,
        command::{Command, CommandHandler},
    },
    clock::{
        countdown::Countdown,
        ticker::{TickHandler, Ticker},
    },
    view::{
        render::{render_text, Renderable},
        util::{chars_height, chars_width},
        viewport::{Coordinates, Viewport},
    },
};
use tui::widgets::canvas::Context;

const ACTIVE_COUNT: u16 = TICKS_PER_SECOND / 10; // 10ms
const MIN_FULL_WIDTH: u16 = 71;

#[derive(Copy, Clone)]
pub enum ButtonSize {
    Condensed,
    Full,
}

pub struct Button {
    active: Countdown,
    coordinates: Coordinates,
    disabled: Countdown,
    disabled_override: bool,
    kind: ButtonKind,
    size: ButtonSize,
}

impl CommandHandler for Button {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        let mut commands = Vec::new();
        match (&self.kind, command) {
            (ButtonKind::Missile, Command::PressMissileButton) => {
                self.maybe_fire(&mut commands, Command::FireMissile)
            }
            (ButtonKind::Missile, Command::UpdateMissiles(current, _)) => {
                self.disabled_override = current == 0;
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

impl Renderable for Button {
    fn render(&mut self, context: &mut Context, viewport: &Viewport) {
        self.size = if viewport.rect.width < MIN_FULL_WIDTH {
            ButtonSize::Condensed
        } else {
            ButtonSize::Full
        };
        render_text(
            context,
            self.coordinates,
            self.kind.text(self.size),
            self.kind.color(
                self.active.on(),
                self.disabled.on() || self.disabled_override,
            ),
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

    pub fn set_coordinates(&mut self, coordinates: Coordinates) {
        self.coordinates = coordinates;
    }

    pub fn height(&self) -> u16 {
        chars_height(self.kind.text(self.size))
    }

    pub fn width(&self) -> u16 {
        chars_width(self.kind.text(self.size))
    }

    fn new(button_kind: ButtonKind) -> Self {
        let disabled_count = button_kind.disabled_count();
        Self {
            active: Countdown::new(ACTIVE_COUNT),
            coordinates: (0, 0), // `ButtonPanel` will update the coordinates before rendering.
            disabled: Countdown::new(disabled_count),
            disabled_override: false,
            kind: button_kind,
            size: ButtonSize::Full,
        }
    }

    fn maybe_fire(&mut self, commands: &mut Vec<Command>, command: Command) {
        if self.disabled.off() && !self.disabled_override {
            self.active.restart();
            self.disabled.restart();
            commands.push(command);
        }
    }
}
