use crate::app::{app::TICKS_PER_SECOND, color::ColorTheme};

use super::button::ButtonSize;

const DISABLED_MISSILE_COUNT: u16 = TICKS_PER_SECOND * 15; // 15 seconds
const DISABLED_SHIELDS_COUNT: u16 = TICKS_PER_SECOND * 30; // 30 seconds

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
        match self {
            Self::Missile => {
                if active {
                    ColorTheme::MissileButtonActive
                } else if disabled {
                    ColorTheme::DisabledButton
                } else {
                    ColorTheme::MissileButton
                }
            }
            Self::Rewind => {
                if active {
                    ColorTheme::RewindButtonActive
                } else if disabled {
                    ColorTheme::DisabledButton
                } else {
                    ColorTheme::RewindButton
                }
            }
            Self::Shields => {
                if active {
                    ColorTheme::ShieldsButtonActive
                } else if disabled {
                    ColorTheme::DisabledButton
                } else {
                    ColorTheme::ShieldsButton
                }
            }
            Self::GameOver => ColorTheme::GameOver,
        }
    }

    pub fn disabled_count(&self) -> u16 {
        match self {
            ButtonKind::Missile => DISABLED_MISSILE_COUNT,
            ButtonKind::Shields => DISABLED_SHIELDS_COUNT,
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
