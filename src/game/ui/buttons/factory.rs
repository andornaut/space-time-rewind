use crate::app::app::TICKS_PER_SECOND;

use super::{
    button::ButtonContainer, game_over::GameOverButton, missile::MissileButton,
    rewind::RewindButton, shields::ShieldsButton,
};

const DISABLED_SHIELDS_COUNT: u16 = TICKS_PER_SECOND * 30; // 30 seconds

pub fn new_game_over() -> ButtonContainer {
    ButtonContainer::new(Box::new(GameOverButton::default()))
}

pub fn new_missiles() -> ButtonContainer {
    ButtonContainer::new(Box::new(MissileButton::default()))
}

pub fn new_rewind() -> ButtonContainer {
    ButtonContainer::new(Box::new(RewindButton::default()))
}

pub fn new_shields() -> ButtonContainer {
    ButtonContainer::new_disableable(Box::new(ShieldsButton::default()), DISABLED_SHIELDS_COUNT)
}
