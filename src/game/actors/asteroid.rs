use crate::{
    app::{
        color::ColorTheme,
        command::{Command, CommandHandler, NO_COMMANDS},
    },
    clock::ticker::{Frequency, TickHandler, Ticker},
    game::game_item::{GameItem, GameItemKind},
    view::{
        render::{render_text, Renderable},
        viewport::{Coordinates, Viewport},
    },
};
use tui::{style::Color, widgets::canvas::Context};

static TEXT_LARGE: &str = "\
\x20▟▒▒▒▓▓▓▒▒▒▓▓▓▓▓▒▓▩\x20\x20\x20\x20\x20
▜▓▓▒▒▒▓▓▓▒▒▓▒▒▓▟▓▓▓▓▓▞\x20\x20
▜▓▓▓▒▒▓▟▓▛▓▓▓▛▛▓▛▛▛▓▓▓▓▞
▜▓▓▓▒▒▓▟▓▓▓▓▓▞▓▓▓▓▓▛\x20\x20\x20\x20
\x20▟▒▒▒▟▒▒▓▓▓▓▓▓▒▓▩\x20\x20\x20\x20\x20\x20\x20
▜▓▓▓▓▓▓▒▛▒▒▓▓▓▒▒▒▒▓▞\x20\x20\x20\x20
\x20\x20\x20▩▒▓▓▓▓▓▓▓▓▒▒▓▛\x20\x20\x20\x20\x20\x20\x20";
static TEXT_MEDIUM: &str = "\
\x20▟▒▒▓▩\x20\x20\x20
▜▓▓▓▓▓▓▓▞
▜▓▓▓▓▓▓▞\x20
\x20▩▒▒▒▓▛\x20\x20";

static TEXT_SMALL: &str = "\
▟▒▓▩
▜▓▓▞
▩▒▓▛";

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum AsteroidKind {
    Large,
    Medium,
    Small,
}

impl AsteroidKind {
    fn frequency(&self) -> Frequency {
        match self {
            AsteroidKind::Large => Frequency::Six,
            AsteroidKind::Medium => Frequency::Five,
            AsteroidKind::Small => Frequency::Four,
        }
    }
    fn initial_hp(&self) -> u8 {
        match self {
            AsteroidKind::Large => 12,
            AsteroidKind::Medium => 6,
            AsteroidKind::Small => 3,
        }
    }

    fn text(&self) -> &'static str {
        match self {
            AsteroidKind::Large => TEXT_LARGE,
            AsteroidKind::Medium => TEXT_MEDIUM,
            AsteroidKind::Small => TEXT_SMALL,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Asteroid {
    coordinates: Coordinates,
    deleted: bool,
    hp: u8,
    kind: AsteroidKind,
}

impl CommandHandler for Asteroid {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        match command {
            Command::Collide(kind) => {
                match kind {
                    GameItemKind::Bullet => self.hp = self.hp.saturating_sub(1),
                    GameItemKind::Missile | GameItemKind::Ship => self.hp = 0,
                    _ => (),
                }
                if self.hp == 0 {
                    self.deleted = true;
                    return vec![Command::AddExplosion(self.viewport().center())];
                }
            }
            _ => (),
        };
        NO_COMMANDS
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
    fn render(&mut self, context: &mut Context, _: Viewport) {
        render_text(context, self.coordinates, self.kind.text(), self.color());
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_from_coordinates(self.width(), self.height(), self.coordinates)
    }
}

impl TickHandler for Asteroid {
    fn handle_tick(&mut self, ticker: &Ticker) {
        if ticker.should(self.kind.frequency()) {
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
    pub fn new_large(coordinates: Coordinates) -> Self {
        Self::new(coordinates, AsteroidKind::Large)
    }

    pub fn new_medium(coordinates: Coordinates) -> Self {
        Self::new(coordinates, AsteroidKind::Medium)
    }

    pub fn new_small(coordinates: Coordinates) -> Self {
        Self::new(coordinates, AsteroidKind::Small)
    }

    fn new(coordinates: Coordinates, kind: AsteroidKind) -> Self {
        Self {
            coordinates,
            deleted: false,
            hp: kind.initial_hp(),
            kind,
        }
    }

    fn color(&self) -> Color {
        if self.hp <= self.kind.initial_hp() / 3 {
            return Color::from(ColorTheme::AsteroidLowHp);
        }
        if self.hp <= (self.kind.initial_hp() as f32 / 1.5) as u8 {
            return Color::from(ColorTheme::AsteroidMidHp);
        }
        Color::from(ColorTheme::AsteroidHighHp)
    }

    fn height(&self) -> u16 {
        self.kind.text().lines().count() as u16
    }

    fn width(&self) -> u16 {
        self.kind.text().lines().next().unwrap().chars().count() as u16
    }
}
