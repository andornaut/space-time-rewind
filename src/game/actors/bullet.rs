use crate::{
    app::{
        color::ColorTheme,
        command::{Command, CommandHandler, NO_COMMANDS},
    },
    clock::ticker::{Frequency, TickHandler, Ticker},
    game::game_item::{GameItem, GameItemKind},
    view::{
        coordinates::Coordinates,
        render::Renderable,
        renderer::Renderer,
        util::{chars_height, chars_width},
        viewport::Viewport,
    },
};

static TEXT: &str = "•";

pub struct Bullet {
    coordinates: Coordinates,
    deleted: bool,
    height: u8,
    width: u8,
}

impl CommandHandler for Bullet {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        if let Command::Collide(kind) = command {
            if kind.is_shootable() {
                self.deleted = true
            }
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
    fn render(&mut self, renderer: &mut Renderer, _: &Viewport) {
        renderer.render_with_offset(self.coordinates, TEXT, ColorTheme::Bullet);
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_with_coordinates(self.width, self.height, self.coordinates)
    }
}

impl TickHandler for Bullet {
    fn handle_tick(&mut self, ticker: &Ticker, world_viewport: &Viewport) {
        if ticker.at(Frequency::One) {
            self.coordinates.y_offset(1);

            if !world_viewport.intersects_vertically(self.viewport()) {
                self.deleted = true;
            }
        }
    }
}

impl Bullet {
    pub fn new(coordinates: Coordinates) -> Self {
        Self {
            coordinates,
            deleted: false,
            height: chars_height(TEXT),
            width: chars_width(TEXT),
        }
    }
}
