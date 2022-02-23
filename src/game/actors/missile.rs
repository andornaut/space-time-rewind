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

static TEXT: &str = "  ▄\x20\x20
▟███▙
▀▜ ▛▀";

pub struct Missile {
    coordinates: Coordinates,
    deleted: bool,
    height: u8,
    width: u8,
}

impl CommandHandler for Missile {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        if let Command::Collide(kind) = command {
            if kind.is_shootable() {
                self.deleted = true
            }
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
    fn render<'a>(&self, renderer: &mut Renderer) {
        renderer.render_with_offset(self.coordinates, TEXT, ColorTheme::Missile);
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_with_coordinates(self.width, self.height, self.coordinates)
    }
}

impl TickHandler for Missile {
    fn handle_tick(&mut self, ticker: &Ticker, world_viewport: Viewport) {
        if ticker.at(Frequency::Two) {
            self.coordinates.y_offset(1);

            if !world_viewport.intersects_vertically(self.viewport()) {
                self.deleted = true;
            }
        }
    }
}

impl Missile {
    pub fn new(mut coordinates: Coordinates) -> Self {
        // The given coordinates are relative to the center of a ship, so left-align.
        let height = chars_height(TEXT);
        let width = chars_width(TEXT);
        coordinates.x_offset(i16::from(width) / -2);
        Self {
            coordinates,
            deleted: false,
            height,
            width,
        }
    }
}
