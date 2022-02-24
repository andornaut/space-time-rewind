use crate::{app::command::CommandHandler, clock::ticker::TickHandler, view::render::Renderable};

pub trait GameItem: CommandHandler + Renderable + TickHandler {
    fn deleted(&self) -> bool {
        false
    }

    fn kind(&self) -> GameItemKind {
        GameItemKind::Unspecified
    }
}

#[derive(Copy, Clone, Debug)]
pub enum GameItemKind {
    Asteroid,
    Bullet,
    Missile,
    Ship,

    Unspecified,
}

impl GameItemKind {
    pub fn is_shootable(self) -> bool {
        matches!(self, GameItemKind::Asteroid)
    }
}
