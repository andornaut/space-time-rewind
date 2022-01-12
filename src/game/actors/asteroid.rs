use tui::{style::Color, widgets::canvas::Context};

use crate::{
    clock::ticker::{Frequency, TickHandler, Ticker},
    color::ColorTheme,
    command::{Command, CommandHandler},
    game::{GameItem, GameItemKind},
    view::{
        renderer::{render_text, Renderable},
        viewport::{Coordinates, Viewport},
    },
};

static TEXT: &str = "\
▟▒▓▩
▜▓▓▞
▩▒▓▛";

#[derive(Clone, Debug)]
pub struct Asteroid {
    coordinates: Coordinates,
    deleted: bool,
    hp: u8,
}

impl CommandHandler for Asteroid {
    fn handle_command(&mut self, command: Command) -> Command {
        match command {
            Command::Collide(kind) => {
                if kind == GameItemKind::Bullet {
                    self.hp = self.hp.saturating_sub(1);
                } else if kind == GameItemKind::Missile {
                    self.hp = 0;
                }
            }
            _ => (),
        };
        if self.hp == 0 {
            self.deleted = true;
        }
        Command::NOOP
    }
}

impl Default for Asteroid {
    fn default() -> Self {
        // TODO Generate different shapes
        Self {
            coordinates: (0, 0),
            deleted: false,
            hp: 3,
        }
    }
}

impl GameItem for Asteroid {
    fn deleted(&self) -> bool {
        self.deleted
    }

    fn kind(&self) -> GameItemKind {
        GameItemKind::Asteroid
    }
}

impl Renderable for Asteroid {
    fn render(&mut self, context: &mut Context, viewport: Viewport) {
        self.coordinates = viewport.contain(&self.viewport());
        render_text(context, self.coordinates, TEXT, self.color());
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_from_coordinates(width(), height(), self.coordinates)
    }
}

impl TickHandler for Asteroid {
    fn handle_tick(&mut self, ticker: &Ticker) {
        if ticker.should(Frequency::Five) {
            let (x, y) = self.coordinates;
            if y == 0 {
                self.deleted = true;
                return;
            }
            self.coordinates = (x, y - 1);
        }
    }
}

impl Asteroid {
    pub fn new(coordinates: Coordinates) -> Self {
        Self {
            coordinates,
            ..Self::default()
        }
    }

    fn color(&self) -> Color {
        match self.hp {
            3 => Color::from(ColorTheme::AsteroidFullHp),
            2 => Color::from(ColorTheme::AsteroidHalfHp),
            _ => Color::from(ColorTheme::AsteroidLowHp),
        }
    }
}

fn height() -> u16 {
    TEXT.lines().count() as u16
}

fn width() -> u16 {
    TEXT.lines().next().unwrap().chars().count() as u16
}
