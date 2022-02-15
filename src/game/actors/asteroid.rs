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

static TEXT_LARGE: &str = "\
\x20\x20▟▒▒▒▓▓▓▒▒▒▓▓▓▓▓▒▓▩
▜▓▓▛▞▒▒▒▓▓▒▓▒▒▓▟▓▓▓▞
▜▓▓▒▒▓▟▛▓▛▛▓▓▛▓▛▛▓▓▞
\x20▜▓▓▓▒▒▓▟▓▓▓▓▞▓▓▓▓▛\x20
\x20\x20▟▒▒▒▛▟▛▒▛▒▓▓▓▓▓▓▒▓▩\x20
\x20▜▓▓▓▓▓▒▛▒▒▓▓▓▒▒▒▒▓▞
▩▒▓▓▓▟▟▓▓▟▟▓▓▓▟▓▓▒▒▓▛";
static TEXT_MEDIUM: &str = "\
\x20▟▒▒▓▩▩\x20\x20\x20
▜▓▓▓▓▞▟▓▞\x20
▜▓▓▞▟▓▓▓▓▞
\x20▩▒▓▒▒▓▛\x20\x20";
static TEXT_SMALL: &str = "\
▟▒▓▩
▜▓▓▞
▩▒▓▛";

enum AsteroidSize {
    Large,
    Medium,
    Small,
}

impl AsteroidSize {
    fn color(&self, hp: u8) -> ColorTheme {
        if hp <= self.initial_hp() / 3 {
            return ColorTheme::AsteroidLowHp;
        }
        if hp <= (self.initial_hp() as f32 / 1.5) as u8 {
            return ColorTheme::AsteroidMidHp;
        }
        match self {
            Self::Large => ColorTheme::AsteroidHighHpLarge,
            Self::Medium => ColorTheme::AsteroidHighHpMedium,
            Self::Small => ColorTheme::AsteroidHighHpSmall,
        }
    }

    fn frequency(&self) -> Frequency {
        match self {
            Self::Large => Frequency::Four,
            Self::Medium => Frequency::Three,
            Self::Small => Frequency::Two,
        }
    }

    fn initial_hp(&self) -> u8 {
        match self {
            Self::Large => 12,
            Self::Medium => 6,
            Self::Small => 3,
        }
    }

    fn points(&self) -> u8 {
        self.initial_hp()
    }

    fn text(&self) -> &'static str {
        match self {
            Self::Large => TEXT_LARGE,
            Self::Medium => TEXT_MEDIUM,
            Self::Small => TEXT_SMALL,
        }
    }
}

pub struct Asteroid {
    coordinates: Coordinates,
    deleted: bool,
    hp: u8,
    kind: AsteroidSize,
}

impl CommandHandler for Asteroid {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        if let Command::Collide(kind) = command {
            match kind {
                GameItemKind::Bullet => self.hp = self.hp.saturating_sub(1),
                GameItemKind::Missile | GameItemKind::Ship => self.hp = 0,
                _ => (),
            }
            if self.hp == 0 {
                self.deleted = true;
                return vec![
                    Command::AddExplosion(self.viewport().centered()),
                    Command::IncreaseScore(self.kind.points()),
                ];
            }
        }
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
    fn render(&mut self, renderer: &mut Renderer, _: &Viewport) {
        renderer.render_with_offset(self.coordinates, self.kind.text(), self.kind.color(self.hp));
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_with_coordinates(self.width(), self.height(), self.coordinates)
    }
}

impl TickHandler for Asteroid {
    fn handle_tick(&mut self, ticker: &Ticker, world_viewport: &Viewport) {
        if ticker.at(self.kind.frequency()) {
            self.coordinates.y_offset(-1);

            if !world_viewport.intersects_vertically(self.viewport()) {
                self.deleted = true;
            }
        }
    }
}

impl Asteroid {
    pub fn new_large(coordinates: Coordinates) -> Self {
        Self::new(coordinates, AsteroidSize::Large)
    }

    pub fn new_medium(coordinates: Coordinates) -> Self {
        Self::new(coordinates, AsteroidSize::Medium)
    }

    pub fn new_small(coordinates: Coordinates) -> Self {
        Self::new(coordinates, AsteroidSize::Small)
    }

    fn new(coordinates: Coordinates, kind: AsteroidSize) -> Self {
        Self {
            coordinates,
            deleted: false,
            hp: kind.initial_hp(),
            kind,
        }
    }

    fn height(&self) -> u8 {
        chars_height(self.kind.text())
    }

    fn width(&self) -> u8 {
        chars_width(self.kind.text())
    }
}
