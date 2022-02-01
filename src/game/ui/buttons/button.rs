use crate::app::{color::ColorTheme, command::Command};

#[derive(Copy, Clone)]
pub enum ButtonSize {
    Condensed,
    Full,
}

pub trait Button {
    fn color(&self, active: bool, disabled: bool) -> ColorTheme;

    fn handle_command(&mut self, _: Command) -> Option<Command> {
        None
    }

    fn text(&self, size: ButtonSize) -> &'static str;
}
