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

static TEXT_HEALTH: &str = "\
┏━━━┓
┃ h ┃
┗━━━┛";
static TEXT_MISSILE: &str = "\
┏━━━┓
┃ m ┃
┗━━━┛";

enum PowerUpKind {
    Health,
    Missile,
}

impl PowerUpKind {
    fn color(&self) -> ColorTheme {
        match self {
            PowerUpKind::Health => ColorTheme::PowerUpHealth,
            PowerUpKind::Missile => ColorTheme::PowerUpMissile,
        }
    }

    fn text(&self) -> &'static str {
        match self {
            PowerUpKind::Health => TEXT_HEALTH,
            PowerUpKind::Missile => TEXT_MISSILE,
        }
    }
}

pub struct PowerUp {
    color: ColorTheme,
    coordinates: Coordinates,
    deleted: bool,
    height: u8,
    kind: PowerUpKind,
    text: &'static str,
    width: u8,
}

impl CommandHandler for PowerUp {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        if let Command::Collide(kind) = command {
            if let GameItemKind::Ship | GameItemKind::ShipWithShields = kind {
                self.deleted = true;
                return match self.kind {
                    PowerUpKind::Health => vec![Command::IncreaseHealth(1)],
                    PowerUpKind::Missile => vec![Command::IncreaseMissiles(1)],
                };
            }
        }
        NO_COMMANDS
    }
}

impl GameItem for PowerUp {
    fn deleted(&self) -> bool {
        self.deleted
    }
}

impl Renderable for PowerUp {
    fn render(&self, renderer: &mut Renderer) {
        renderer.render_with_offset(self.coordinates, self.text, self.color);
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_with_coordinates(self.width, self.height, self.coordinates)
    }
}

impl TickHandler for PowerUp {
    fn handle_tick(&mut self, ticker: &Ticker, world_viewport: Viewport) {
        if ticker.at(Frequency::Three) {
            self.coordinates.y_offset(-1);

            if !world_viewport.intersects_vertically(self.viewport()) {
                self.deleted = true;
            }
        }
    }
}

impl PowerUp {
    pub fn new_health(coordinates: Coordinates) -> Self {
        Self::new(coordinates, PowerUpKind::Health)
    }

    pub fn new_missile(coordinates: Coordinates) -> Self {
        Self::new(coordinates, PowerUpKind::Missile)
    }

    fn new(coordinates: Coordinates, kind: PowerUpKind) -> Self {
        let text = kind.text();
        Self {
            color: kind.color(),
            coordinates,
            deleted: false,
            height: chars_height(text),
            kind,
            text,
            width: chars_width(text),
        }
    }
}
