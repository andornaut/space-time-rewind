use crate::{app::command::CommandHandler, clock::ticker::TickHandler, view::render::Renderable};

pub trait GameItem: CommandHandler + Renderable + TickHandler {
    fn deleted(&self) -> bool {
        false
    }

    fn kind(&self) -> GameItemKind {
        GameItemKind::Unspecified
    }
}

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
