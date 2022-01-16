use tui::{style::Color, widgets::canvas::Context};

use crate::{
    clock::{
        countdown::Countdown,
        ticker::{TickHandler, Ticker},
    },
    color::ColorTheme,
    command::{Command, CommandHandler},
    game::{GameItem, GameItemKind},
    view::{
        render::{render_text, Renderable},
        viewport::{Coordinates, Viewport},
    },
};

static TEXT: &str = "◄◆►";

const DISABLED_GUNS_COUNT: u16 = 5;
pub const DISABLED_MISSILE_COUNT: u16 = 600; // The Missile `Button` needs to use the same value.

#[derive(Clone, Debug)]
pub struct Ship {
    coordinates: Coordinates,
    deleted: bool,
    disabled_guns: Countdown,
    disabled_missile: Countdown,
    initialized: bool,
}

impl CommandHandler for Ship {
    fn handle_command(&mut self, command: Command) -> Command {
        match command {
            Command::Collide(kind) => {
                if !kind.is_weapon() {
                    self.deleted = true;
                    return Command::GameOver;
                }
            }
            Command::FireGuns => {
                if self.disabled_guns.off() {
                    self.disabled_guns.restart();
                    let (cx, cy) = self.viewport().center();
                    return Command::AddBullet((cx, cy + 1));
                }
            }
            Command::FireMissile => {
                if self.disabled_missile.off() {
                    self.disabled_missile.restart();
                    let (cx, cy) = self.viewport().center();
                    return Command::AddMissile((cx, cy + 1));
                }
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
        Command::NOOP
    }
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            coordinates: (0, 0),
            deleted: false,
            disabled_guns: Countdown::new(DISABLED_GUNS_COUNT),
            disabled_missile: Countdown::new(DISABLED_MISSILE_COUNT),
            initialized: false,
        }
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
        if !self.initialized {
            self.initialized = true;
            // Center on first render, because this is the first time that we have the viewport dimensions.
            self.coordinates = viewport.center();
        }
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
        self.disabled_missile.down();
    }
}

fn height() -> u16 {
    TEXT.lines().count() as u16
}

fn width() -> u16 {
    TEXT.lines().next().unwrap().chars().count() as u16
}
