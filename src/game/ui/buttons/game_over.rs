use super::button::{Button, ButtonSize};
use crate::app::color::ColorTheme;

static TEXT: &str = "\
Game over!   \x20
Press [r] to restart or [q] to quit "; // Add whitespace to the first line to overwrite the health bar

#[derive(Default)]
pub struct GameOverButton {}

impl Button for GameOverButton {
    fn color(&self, _: bool, _: bool) -> ColorTheme {
        ColorTheme::GameOver
    }

    fn text(&self, _: ButtonSize) -> &'static str {
        TEXT
    }
}
