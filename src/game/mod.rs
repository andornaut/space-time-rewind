use crate::{clock::ticker::TickHandler, command::CommandHandler, view::renderer::Renderable};

mod actors;
mod buttons;
mod spawner;
pub mod world;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameItemKind {
    Asteroid,
    Bullet,
    Button,
    Missile,
    Ship,

    Unspecified,
}

impl GameItemKind {
    pub fn is_shootable(self) -> bool {
        match self {
            GameItemKind::Asteroid => true,
            _ => false,
        }
    }

    pub fn is_weapon(self) -> bool {
        match self {
            GameItemKind::Bullet | GameItemKind::Missile => true,
            _ => false,
        }
    }
}

pub trait GameItem: CommandHandler + Renderable + TickHandler {
    fn deleted(&self) -> bool {
        false
    }

    fn kind(&self) -> GameItemKind {
        GameItemKind::Unspecified
    }
}
