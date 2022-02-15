use super::button::{Button, ButtonSize};
use crate::app::color::ColorTheme;

static TEXT: &str = "\
==============================================
Game over! Press [r] to restart or [q] to quit
==============================================";

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
