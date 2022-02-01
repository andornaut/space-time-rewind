use super::button::{Button, ButtonSize};
use crate::app::color::ColorTheme;

static TEXT: &str = "\
==============================================
Game over! Press [r] to restart or [q] to quit
==============================================";

pub struct GameOverButton {}

impl Default for GameOverButton {
    fn default() -> Self {
        Self {}
    }
}

impl Button for GameOverButton {
    fn color(&self, _: bool, _: bool) -> ColorTheme {
        ColorTheme::GameOver
    }

    fn text(&self, _: ButtonSize) -> &'static str {
        TEXT
    }
}
