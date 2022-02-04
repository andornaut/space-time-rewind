use crate::{
    app::{app::TICKS_PER_SECOND, color::ColorTheme, command::CommandHandler},
    clock::{
        countdown::Countdown,
        ticker::{TickHandler, Ticker},
    },
    game::game_item::GameItem,
    view::{
        render::{render_text, Renderable},
        util::{chars_height, chars_width},
        viewport::{Coordinates, Viewport},
    },
};
use tui::widgets::canvas::Context;

const LIFECYCLE_TRANSITION_COUNT: u16 = TICKS_PER_SECOND / 16; // 62.5ms

static TEXT_A: &str = "\
\x20\x20\x20\x20\x20\x20
\x20\x20▒▒\x20\x20
\x20\x20\x20\x20\x20\x20";

static TEXT_B: &str = "\
\x20\x20░░\x20\x20
\x20░▒▒░\x20
\x20\x20░░\x20\x20";

static TEXT_C_AND_D: &str = "\
░▒▒▒░
▒░░░▒
░▒▒▒░";

enum Animation {
    A,
    B,
    C,
    D,
    Deleted,
}

impl Animation {
    fn color(&self) -> ColorTheme {
        match self {
            Self::A => ColorTheme::ExplosionA,
            Self::B => ColorTheme::ExplosionB,
            Self::C => ColorTheme::ExplosionC,
            Self::D => ColorTheme::ExplosionD,
            Self::Deleted => panic!("Cannot invoke methods on Lifecycle::Deleted"),
        }
    }

    fn next(&self) -> Self {
        match self {
            Self::A => Self::B,
            Self::B => Self::C,
            Self::C => Self::D,
            Self::D => Self::Deleted,
            Self::Deleted => panic!("Cannot advance past Lifecycle::Deleted"),
        }
    }

    fn text(&self) -> &'static str {
        match self {
            Self::A => TEXT_A,
            Self::B => TEXT_B,
            Self::C => TEXT_C_AND_D,
            Self::D => TEXT_C_AND_D,
            Self::Deleted => panic!("Cannot invoke methods on Lifecycle::Deleted"),
        }
    }
}

pub struct Explosion {
    coordinates: Coordinates,
    deleted: bool,
    animation: Animation,
    animation_next: Countdown,
}

impl CommandHandler for Explosion {}

impl GameItem for Explosion {
    fn deleted(&self) -> bool {
        self.deleted
    }
}

impl Renderable for Explosion {
    fn render(&mut self, context: &mut Context, _: &Viewport) {
        render_text(
            context,
            self.coordinates,
            self.animation.text(),
            self.animation.color(),
        );
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_from_coordinates(self.width(), self.height(), self.coordinates)
    }
}

impl TickHandler for Explosion {
    fn handle_tick(&mut self, _: &Ticker) {
        if self.animation_next.off() {
            self.animation_next.restart();
            self.animation = self.animation.next();
            if let Animation::Deleted = self.animation {
                self.deleted = true;
            }
        }

        self.animation_next.down();
    }
}

impl Explosion {
    pub fn new(coordinates: Coordinates) -> Self {
        let mut lifecycle_next = Countdown::new(LIFECYCLE_TRANSITION_COUNT);
        lifecycle_next.restart();
        let mut obj = Self {
            coordinates,
            deleted: false,
            animation: Animation::A,
            animation_next: lifecycle_next,
        };
        obj.coordinates = obj.viewport().centered_around_bottom_left();
        obj
    }

    fn height(&self) -> u16 {
        chars_height(self.animation.text())
    }

    fn width(&self) -> u16 {
        chars_width(self.animation.text())
    }
}
