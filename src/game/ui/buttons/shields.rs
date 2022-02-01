use super::button::{Button, ButtonSize};
use crate::app::{color::ColorTheme, command::Command};

static TEXT_CONDENSED: &str = "\
╭───╮
│[k]│
╰───╯";
static TEXT_FULL: &str = "\
╭───────────╮
│Shields [k]│
╰───────────╯";

pub struct ShieldsButton {}

impl Default for ShieldsButton {
    fn default() -> Self {
        Self {}
    }
}

impl Button for ShieldsButton {
    fn color(&self, active: bool, disabled: bool) -> ColorTheme {
        if active {
            ColorTheme::ShieldsButtonActive
        } else if disabled {
            ColorTheme::DisabledButton
        } else {
            ColorTheme::ShieldsButton
        }
    }

    fn handle_command(&mut self, command: Command) -> Option<Command> {
        if let Command::PressShieldsButton = command {
            Some(Command::FireShields)
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
