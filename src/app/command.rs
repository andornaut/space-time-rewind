use crate::{
    game::game_item::GameItemKind,
    view::{
        coordinates::{Coordinates, Movement},
        viewport::Viewport,
    },
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub const NO_COMMANDS: Vec<Command> = Vec::new();

pub trait CommandHandler {
    fn handle_command(&mut self, _: Command) -> Vec<Command> {
        NO_COMMANDS
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Command {
    Continue,
    GameOver, // Display a prompt to quit or restart
    Quit,     // Exit the application
    Restart,

    // Actors
    Collide(GameItemKind),
    AddBullet(Coordinates),
    AddExplosion(Coordinates),
    AddMissile(Coordinates),
    MoveOffset(Movement),
    MoveShip(Movement),

    // Button commands
    PressMissileButton,
    PressShieldsButton,

    // Ship commands
    FireGuns,
    FireMissile,
    FireShields,

    // UI
    IncreaseHealth(u8),
    IncreaseMissiles(u8),
    IncreaseScore(u32),
    UpdateHealth(u8, u8),
    UpdateMissiles(u8, u8),

    // Viewports
    ActorsViewportChanged(Viewport),
    ActorsViewportInitialized(Viewport),
    UiViewportInitializedOrChanged(Viewport),
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

            KeyCode::Char('q') => Self::Quit,
            KeyCode::Char('r') => Self::Restart,
            _ => Command::Continue,
        }
    }
}
