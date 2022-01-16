use tui::{style::Color, widgets::canvas::Context};

use crate::{
    clock::{
        countdown::Countdown,
        ticker::{TickHandler, Ticker},
    },
    color::ColorTheme,
    command::CommandHandler,
    game::game_item::GameItem,
    view::{
        render::{render_text, Renderable},
        viewport::{Coordinates, Viewport},
    },
};

const LIFECYCLE_TRANSITION_COUNT: u16 = 3;

static TEXT_START: &str = "\
\x20\x20\x20\x20
\x20▒▒\x20
\x20\x20\x20\x20";

static TEXT_MIDDLE: &str = "\
\x20░░\x20
░▒▒░
\x20░░\x20";

static TEXT_END: &str = "\
░▒▒░
▒░░▒
░▒▒░";

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Lifecycle {
    Start,
    Middle,
    End,
    End2,
    Deleted,
}

impl Lifecycle {
    fn color(&self) -> Color {
        match self {
            Self::Start => Color::from(ColorTheme::ExplosionStart),
            Self::Middle => Color::from(ColorTheme::ExplosionMiddle),
            Self::End => Color::from(ColorTheme::ExplosionEnd),
            Self::End2 => Color::from(ColorTheme::ExplosionEnd2),
            Self::Deleted => panic!("Cannot invoke methods on Lifecycle::Deleted"),
        }
    }

    fn next(&self) -> Self {
        match self {
            Self::Start => Self::Middle,
            Self::Middle => Self::End,
            Self::End => Self::End2,
            Self::End2 => Self::Deleted,
            Self::Deleted => panic!("Cannot advance past Lifecycle::Deleted"),
        }
    }

    fn text(&self) -> &'static str {
        match self {
            Self::Start => TEXT_START,
            Self::Middle => TEXT_MIDDLE,
            Self::End => TEXT_END,
            Self::End2 => TEXT_END,
            Self::Deleted => panic!("Cannot invoke methods on Lifecycle::Deleted"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Explosion {
    coordinates: Coordinates,
    deleted: bool,
    lifecycle: Lifecycle,
    lifecycle_next: Countdown,
}

impl CommandHandler for Explosion {}

impl GameItem for Explosion {
    fn deleted(&self) -> bool {
        self.deleted
    }
}

impl Renderable for Explosion {
    fn render(&mut self, context: &mut Context, _: Viewport) {
        render_text(
            context,
            self.coordinates,
            self.lifecycle.text(),
            self.lifecycle.color(),
        );
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_from_coordinates(width(), height(), self.coordinates)
    }
}

impl TickHandler for Explosion {
    fn handle_tick(&mut self, _: &Ticker) {
        if self.lifecycle_next.off() {
            self.lifecycle_next.restart();
            self.lifecycle = self.lifecycle.next();
            if let Lifecycle::Deleted = self.lifecycle {
                self.deleted = true;
            }
        }

        self.lifecycle_next.down();
    }
}

impl Explosion {
    pub fn new(coordinates: Coordinates) -> Self {
        let mut lifecycle_next = Countdown::new(LIFECYCLE_TRANSITION_COUNT);
        lifecycle_next.restart();
        let mut obj = Self {
            coordinates,
            deleted: false,
            lifecycle: Lifecycle::Start,
            lifecycle_next,
        };
        obj.coordinates = obj.viewport().centered_around_bottom_left();
        obj
    }
}

fn height() -> u16 {
    TEXT_START.lines().count() as u16
}

fn width() -> u16 {
    TEXT_START.lines().next().unwrap().chars().count() as u16
}
