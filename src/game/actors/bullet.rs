use tui::{style::Color, widgets::canvas::Context};

use crate::{
    clock::ticker::{Frequency, TickHandler, Ticker},
    color::ColorTheme,
    command::{Command, CommandHandler},
    game::{GameItem, GameItemKind},
    view::{
        render::{render_text, Renderable},
        viewport::{Coordinates, Viewport},
    },
};

static TEXT: &str = "•";

#[derive(Clone, Debug)]
pub struct Bullet {
    coordinates: Coordinates,
    deleted: bool,
}

impl CommandHandler for Bullet {
    fn handle_command(&mut self, command: Command) -> Command {
        match command {
            Command::Collide(kind) => {
                if kind.is_shootable() {
                    self.deleted = true
                }
            }
            _ => (),
        }
        Command::NOOP
    }
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            coordinates: (0, 0),
            deleted: false,
        }
    }
}

impl GameItem for Bullet {
    fn deleted(&self) -> bool {
        self.deleted
    }

    fn kind(&self) -> GameItemKind {
        GameItemKind::Bullet
    }
}

impl Renderable for Bullet {
    fn render(&mut self, context: &mut Context, viewport: Viewport) {
        if viewport.out_of_bounds_completely(&self.viewport()) {
            self.deleted = true;
            return;
        }
        render_text(
            context,
            self.coordinates,
            TEXT,
            Color::from(ColorTheme::Bullet),
        );
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_from_coordinates(width(), height(), self.coordinates)
    }
}

impl TickHandler for Bullet {
    fn handle_tick(&mut self, ticker: &Ticker) {
        if ticker.should(Frequency::Four) {
            let (x, y) = self.coordinates;
            self.coordinates = (x, y + 1);
        }
    }
}

impl Bullet {
    pub fn new(coordinates: Coordinates) -> Self {
        Self {
            coordinates,
            ..Self::default()
        }
    }
}

fn height() -> u16 {
    TEXT.lines().count() as u16
}

fn width() -> u16 {
    TEXT.lines().next().unwrap().chars().count() as u16
}
