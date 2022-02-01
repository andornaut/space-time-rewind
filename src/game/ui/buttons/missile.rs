use super::button::{Button, ButtonSize};
use crate::app::{color::ColorTheme, command::Command};

static TEXT_CONDENSED: &str = "\
╭───╮
│[j]│
╰───╯";
static TEXT_FULL: &str = "\
╭───────────╮
│Missile [j]│
╰───────────╯";

pub struct MissileButton {
    disabled: bool,
}

impl Default for MissileButton {
    fn default() -> Self {
        Self { disabled: false }
    }
}

impl Button for MissileButton {
    fn color(&self, active: bool, disabled: bool) -> ColorTheme {
        if disabled || self.disabled {
            ColorTheme::DisabledButton
        } else if active {
            ColorTheme::MissileButtonActive
        } else {
            ColorTheme::MissileButton
        }
    }

    fn handle_command(&mut self, command: Command) -> Option<Command> {
        if let Command::UpdateMissiles(current, _) = command {
            self.disabled = current == 0;
        }
        if self.disabled {
            return None;
        }
        if let Command::PressMissileButton = command {
            Some(Command::FireMissile)
        } else {
            None
        }
    }

    fn text(&self, size: ButtonSize) -> &'static str {
        match size {
            ButtonSize::Condensed => TEXT_CONDENSED,
            ButtonSize::Full => TEXT_FULL,
        }
    }
}
