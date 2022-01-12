use tui::style::Color;

use crate::{color::ColorTheme, game::actors::ship::DISABLED_MISSILE_COUNT};

static TEXT_GAME_OVER: &str = "\
==============================================
Game over! Press [r] to restart or [q] to quit
==============================================";
static TEXT_MISSILE: &str = "\
╭───────────╮
│Missile [j]│
╰───────────╯";
static TEXT_REWIND: &str = "\
╭───────────╮
│Rewind! [l]│
╰───────────╯";
static TEXT_SHIELDS: &str = "\
╭───────────╮
│Shields [k]│
╰───────────╯";

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonKind {
    GameOver,
    Missile,
    Rewind,
    Shields,
}

impl ButtonKind {
    pub fn color(&self, active: bool, disabled: bool) -> Color {
        if disabled {
            return Color::from(ColorTheme::DisabledButton);
        }
        Color::from(match self {
            ButtonKind::Missile => {
                if active {
                    ColorTheme::MissileActive
                } else {
                    ColorTheme::Missile
                }
            }
            ButtonKind::Rewind => {
                if active {
                    ColorTheme::RewindActive
                } else {
                    ColorTheme::Rewind
                }
            }
            ButtonKind::Shields => {
                if active {
                    ColorTheme::ShieldActive
                } else {
                    ColorTheme::Shield
                }
            }
            ButtonKind::GameOver => ColorTheme::GameOver,
        })
    }

    pub fn disabled_count(&self) -> u16 {
        match self {
            // Accord with the value used by the `Ship`.
            // Eventually other button kinds will have their own starting values.
            ButtonKind::Missile => DISABLED_MISSILE_COUNT,
            _ => 0, // `self.disabled` will always be off if initialized to 0.
        }
    }

    pub fn text(&self) -> &'static str {
        match self {
            ButtonKind::GameOver => TEXT_GAME_OVER,
            ButtonKind::Missile => TEXT_MISSILE,
            ButtonKind::Rewind => TEXT_REWIND,
            ButtonKind::Shields => TEXT_SHIELDS,
        }
    }
}
