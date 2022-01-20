use crate::app::color::ColorTheme;

use super::button::ButtonSize;

const DISABLED_MISSILE_COUNT: u16 = 600; // The Missile `Button` needs to use the same value.
static TEXT_GAME_OVER: &str = "\
==============================================
Game over! Press [r] to restart or [q] to quit
==============================================";
static TEXT_MISSILE_CONDENSED: &str = "\
╭───╮
│[j]│
╰───╯";
static TEXT_MISSILE_FULL: &str = "\
╭───────────╮
│Missile [j]│
╰───────────╯";
static TEXT_REWIND_CONDENSED: &str = "\
╭───╮
│[l]│
╰───╯";
static TEXT_REWIND_FULL: &str = "\
╭───────────╮
│Rewind! [l]│
╰───────────╯";
static TEXT_SHIELDS_CONDENSED: &str = "\
╭───╮
│[k]│
╰───╯";
static TEXT_SHIELDS_FULL: &str = "\
╭───────────╮
│Shields [k]│
╰───────────╯";

pub enum ButtonKind {
    GameOver,
    Missile,
    Rewind,
    Shields,
}

impl ButtonKind {
    pub fn color(&self, active: bool, disabled: bool) -> ColorTheme {
        if disabled {
            return ColorTheme::DisabledButton;
        }
        match self {
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
        }
    }

    pub fn disabled_count(&self) -> u16 {
        match self {
            // Eventually other button kinds will have their own starting values.
            ButtonKind::Missile => DISABLED_MISSILE_COUNT,
            _ => 0, // `self.disabled` will always be off if initialized to 0.
        }
    }

    pub fn text(&self, size: ButtonSize) -> &'static str {
        match (self, size) {
            (ButtonKind::GameOver, _) => TEXT_GAME_OVER,
            (ButtonKind::Missile, ButtonSize::Condensed) => TEXT_MISSILE_CONDENSED,
            (ButtonKind::Missile, ButtonSize::Full) => TEXT_MISSILE_FULL,
            (ButtonKind::Rewind, ButtonSize::Condensed) => TEXT_REWIND_CONDENSED,
            (ButtonKind::Rewind, ButtonSize::Full) => TEXT_REWIND_FULL,
            (ButtonKind::Shields, ButtonSize::Condensed) => TEXT_SHIELDS_CONDENSED,
            (ButtonKind::Shields, ButtonSize::Full) => TEXT_SHIELDS_FULL,
        }
    }
}
