use crate::{
    game::game_item::GameItemKind,
    view::viewport::{Coordinates, Movement},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub const NO_COMMANDS: Vec<Command> = Vec::new();

pub trait CommandHandler {
    fn handle_command(&mut self, _: Command) -> Vec<Command> {
        NO_COMMANDS
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Continue,
    GameOver, // Display an option to quit or restart
    Quit,     // Exit the application
    Restart,

    // Actors
    Collide(GameItemKind),
    AddBullet(Coordinates),
    AddExplosion(Coordinates),
    AddMissile(Coordinates),
    MoveShip(Movement),

    // Button commands
    PressMissileButton,
    PressRewindButton,
    PressShieldsButton,
    // Ship commands
    FireGuns,
    FireMissile,
    FireRewind,
    FireShields,
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

            KeyCode::Char(' ') => Self::FireGuns,
            KeyCode::Char('j') => Self::PressMissileButton,
            KeyCode::Char('k') => Self::PressShieldsButton,
            KeyCode::Char('l') => Self::PressRewindButton,

            KeyCode::Char('q') => Self::Quit,
            KeyCode::Char('r') => Self::Restart,
            _ => Command::Continue,
        }
    }
}