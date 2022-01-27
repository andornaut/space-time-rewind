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
    coordinates: Coordinates,
    color: ColorTheme,
    deleted: bool,
    height: u16,
    kind: PowerUpKind,
    text: &'static str,
    width: u16,
}

impl CommandHandler for PowerUp {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        if let Command::Collide(kind) = command {
            if let GameItemKind::Ship | GameItemKind::Shields = kind {
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
    fn render(&mut self, context: &mut Context, _: &Viewport) {
        render_text(context, self.coordinates, self.text, self.color);
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_from_coordinates(self.width, self.height, self.coordinates)
    }
}

impl TickHandler for PowerUp {
    fn handle_tick(&mut self, ticker: &Ticker) {
        if ticker.at(Frequency::Five) {
            let (x, y) = self.coordinates;
            if y == 0 {
                self.deleted = true;
                return;
            }
            self.coordinates = (x, y - 1);
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
            coordinates,
            color: kind.color(),
            height: chars_height(text),
            deleted: false,
            kind,
            text,
            width: chars_width(text),
        }
    }
}
