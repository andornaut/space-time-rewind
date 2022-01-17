use crate::{
    app::{
        color::ColorTheme,
        command::{Command, CommandHandler, NO_COMMANDS},
    },
    clock::{
        countdown::Countdown,
        ticker::{TickHandler, Ticker},
    },
    game::game_item::{GameItem, GameItemKind},
    view::{
        render::{render_text, Renderable},
        viewport::{Coordinates, Viewport},
    },
};
use tui::{style::Color, widgets::canvas::Context};

static TEXT: &str = "◄◆►";

const DISABLED_GUNS_COUNT: u16 = 5;

#[derive(Clone, Debug)]
pub struct Ship {
    coordinates: Coordinates,
    deleted: bool,
    disabled_guns: Countdown,
}

impl CommandHandler for Ship {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        match command {
            Command::Collide(kind) => {
                if !kind.is_weapon() {
                    self.deleted = true;
                    return vec![Command::GameOver];
                }
            }
            Command::FireGuns => {
                if self.disabled_guns.off() {
                    self.disabled_guns.restart();
                    let (cx, cy) = self.viewport().center();
                    return vec![Command::AddBullet((cx, cy + 1))];
                }
            }
            Command::FireMissile => {
                let (cx, cy) = self.viewport().center();
                return vec![Command::AddMissile((cx, cy + 1))];
            }
            Command::MoveShip((dx, dy)) => {
                let (x, y) = self.coordinates;
                let width = i16::try_from(width()).unwrap();
                self.coordinates = (
                    u16::from(x.saturating_add_signed(dx * width)),
                    u16::from(y.saturating_add_signed(dy)),
                );
            }
            _ => (),
        }
        NO_COMMANDS
    }
}

impl GameItem for Ship {
    fn deleted(&self) -> bool {
        self.deleted
    }

    fn kind(&self) -> GameItemKind {
        GameItemKind::Ship
    }
}

impl Renderable for Ship {
    fn render(&mut self, context: &mut Context, viewport: Viewport) {
        // Prevent the ship from going out of bounds when the viewport is resized.
        self.coordinates = viewport.contain(&self.viewport());
        render_text(
            context,
            self.coordinates,
            TEXT,
            Color::from(ColorTheme::Ship),
        );
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_from_coordinates(width(), height(), self.coordinates)
    }
}

impl TickHandler for Ship {
    fn handle_tick(&mut self, _: &Ticker) {
        self.disabled_guns.down();
    }
}

impl Ship {
    pub fn new(coordinates: Coordinates) -> Self {
        Self {
            coordinates,
            deleted: false,
            disabled_guns: Countdown::new(DISABLED_GUNS_COUNT),
        }
    }
}

fn height() -> u16 {
    TEXT.lines().count() as u16
}

fn width() -> u16 {
    TEXT.lines().next().unwrap().chars().count() as u16
}
