use super::button::{Button, ButtonSize};
use crate::app::{color::ColorTheme, command::Command};

static TEXT_CONDENSED: &str = "\
╭───╮
│[l]│
╰───╯";
static TEXT_FULL: &str = "\
╭───────────╮
│Rewind! [l]│
╰───────────╯";

#[derive(Default)]
pub struct RewindButton {}

impl Button for RewindButton {
    fn color(&self, active: bool, disabled: bool) -> ColorTheme {
        if active {
            ColorTheme::RewindButtonActive
        } else if disabled {
            ColorTheme::DisabledButton
        } else {
            ColorTheme::RewindButton
        }
    }

    fn handle_command(&mut self, command: Command) -> Option<Command> {
        if let Command::PressRewindButton = command {
            Some(Command::FireRewind)
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
