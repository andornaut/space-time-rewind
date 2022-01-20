use crate::{
    app::{
        color::ColorTheme,
        command::{Command, CommandHandler, NO_COMMANDS},
    },
    clock::ticker::{Frequency, TickHandler, Ticker},
    game::game_item::{GameItem, GameItemKind},
    view::{
        render::{render_text, Renderable},
        util::{chars_height, chars_width},
        viewport::{Coordinates, Viewport},
    },
};
use tui::widgets::canvas::Context;

// \x20 is a quoted space
static TEXT: &str = "\
\x20/▲\\\x20
◊╬╬╬◊
 ▾▾▾\x20";

pub struct Missile {
    coordinates: Coordinates,
    deleted: bool,
}

impl CommandHandler for Missile {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        match command {
            Command::Collide(kind) => {
                if kind.is_shootable() {
                    self.deleted = true
                }
            }
            _ => (),
        }
        NO_COMMANDS
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
    fn render<'a>(&mut self, context: &mut Context, viewport: &Viewport) {
        if viewport.out_of_bounds_completely(&self.viewport()) {
            self.deleted = true;
            return;
        }
        render_text(context, self.coordinates, TEXT, ColorTheme::Missile);
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_from_coordinates(width(), height(), self.coordinates)
    }
}

impl TickHandler for Missile {
    fn handle_tick(&mut self, ticker: &Ticker) {
        if ticker.at(Frequency::Five) {
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
    chars_height(TEXT)
}

fn width() -> u16 {
    chars_width(TEXT)
}
