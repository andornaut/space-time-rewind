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

static TEXT: &str = "•";

pub struct Bullet {
    coordinates: Coordinates,
    deleted: bool,
}

impl CommandHandler for Bullet {
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

impl GameItem for Bullet {
    fn deleted(&self) -> bool {
        self.deleted
    }

    fn kind(&self) -> GameItemKind {
        GameItemKind::Bullet
    }
}

impl Renderable for Bullet {
    fn render(&mut self, context: &mut Context, viewport: &Viewport) {
        if viewport.out_of_bounds_completely(&self.viewport()) {
            self.deleted = true;
            return;
        }
        render_text(context, self.coordinates, TEXT, ColorTheme::Bullet);
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_from_coordinates(width(), height(), self.coordinates)
    }
}

impl TickHandler for Bullet {
    fn handle_tick(&mut self, ticker: &Ticker) {
        if ticker.at(Frequency::Four) {
            let (x, y) = self.coordinates;
            self.coordinates = (x, y + 1);
        }
    }
}

impl Bullet {
    pub fn new(coordinates: Coordinates) -> Self {
        Self {
            coordinates,
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
