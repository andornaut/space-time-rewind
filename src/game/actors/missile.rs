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

// \x20 is a quoted space
static TEXT: &str = "\
\x20/▲\\\x20
◊╬╬╬◊
 ▾▾▾\x20";

#[derive(Clone, Debug)]
pub struct Missile {
    coordinates: Coordinates,
    deleted: bool,
}

impl CommandHandler for Missile {
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

impl GameItem for Missile {
    fn deleted(&self) -> bool {
        self.deleted
    }

    fn kind(&self) -> GameItemKind {
        GameItemKind::Missile
    }
}

impl Renderable for Missile {
    fn render<'a>(&mut self, context: &mut Context, viewport: Viewport) {
        if viewport.out_of_bounds_completely(&self.viewport()) {
            self.deleted = true;
            return;
        }
        render_text(
            context,
            self.coordinates,
            TEXT,
            Color::from(ColorTheme::Missile),
        );
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_from_coordinates(width(), height(), self.coordinates)
    }
}

impl TickHandler for Missile {
    fn handle_tick(&mut self, ticker: &Ticker) {
        if ticker.should(Frequency::Five) {
            let (x, y) = self.coordinates;
            self.coordinates = (x, y + 1);
        }
    }
}

impl Missile {
    pub fn new(coordinates: Coordinates) -> Self {
        // The given coordinates are relative to the center of a ship, so left-align.
        let (x, y) = coordinates;
        let x = x.saturating_sub(width().saturating_div(2));
        Self {
            coordinates: (x, y),
            deleted: false,
        }
    }
}

fn height() -> u16 {
    TEXT.lines().count() as u16
}

fn width() -> u16 {
    TEXT.lines().next().unwrap().chars().count() as u16
}
