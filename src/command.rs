use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    game::GameItemKind,
    view::viewport::{Coordinates, Movement},
};

pub trait CommandHandler {
    fn handle_command(&mut self, _: Command) -> Command {
        Command::NOOP
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Command {
    // App states
    NOOP,
    GameOver,

    // Actors
    Collide(GameItemKind),
    AddBullet(Coordinates),
    AddExplosion(Coordinates),
    AddMissile(Coordinates),
    MoveShip(Movement),

    // Ship controls
    ActivateShields,
    FireGuns,
    FireMissile,
    Rewind,

    // Game controls
    Quit,
    Restart,
}

impl From<KeyEvent> for Command {
    fn from(event: KeyEvent) -> Self {
        let KeyEvent { code, modifiers } = event;
        match (code, modifiers) {
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => return Self::Quit,
            (_, _) => (),
        };
        match code {
            KeyCode::Down | KeyCode::Char('s') => Self::MoveShip((0, -1)),
            KeyCode::Up | KeyCode::Char('w') => Self::MoveShip((0, 1)),
            KeyCode::Right | KeyCode::Char('d') => Self::MoveShip((1, 0)),
            KeyCode::Left | KeyCode::Char('a') => Self::MoveShip((-1, 0)),

            KeyCode::Char('k') => Self::ActivateShields,
            KeyCode::Char(' ') => Self::FireGuns,
            KeyCode::Char('j') => Self::FireMissile,
            KeyCode::Char('l') => Self::Rewind,

            KeyCode::Char('q') => Self::Quit,
            KeyCode::Char('r') => Self::Restart,
            _ => Command::NOOP,
        }
    }
}
