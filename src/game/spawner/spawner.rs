use super::levels::{initial, level1};
use crate::{
    clock::ticker::Ticker,
    game::{
        game_item::GameItem,
        ui::{
            buttons::panel::ButtonPanel, health_bar::HealthBar, missiles_bar::MissilesBar,
            score::Score,
        },
    },
    view::viewport::Viewport,
};

pub enum Level {
    Initial,
    Level1,
}

impl Level {
    // TODO Remove the following #[allow] once the TODO in the method body is addressed.
    #[allow(clippy::match_single_binding)]
    fn next(&mut self, _: &Ticker) {
        *self = match self {
            // TODO Advance levels. Consider using `Countdown` or `Ticker` to time the advancement.
            _ => Self::Level1,
        }
    }

    fn spawn(&self, ticker: &Ticker, world_viewport: Viewport) -> Vec<Box<dyn GameItem>> {
        match self {
            Self::Initial => initial(world_viewport),
            Self::Level1 => level1(ticker, world_viewport),
        }
    }
}

pub struct Spawner {
    level: Level,
}

impl Default for Spawner {
    fn default() -> Self {
        Spawner {
            level: Level::Initial,
        }
    }
}

impl Spawner {
    pub fn actors(&mut self, ticker: &Ticker, world_viewport: Viewport) -> Vec<Box<dyn GameItem>> {
        let actors = self.level.spawn(ticker, world_viewport);
        self.level.next(ticker);
        actors
    }

    pub fn ui(&self) -> Vec<Box<dyn GameItem>> {
        vec![
            Box::new(HealthBar::default()),
            Box::new(MissilesBar::default()),
            Box::new(Score::default()),
            // Render the health, missiles, and score UIs before the button panel, so that they'll
            // be rendered below the panel when the viewport is very narrow.
            Box::new(ButtonPanel::default()),
        ]
    }

    pub fn restart(&mut self) {
        self.level = Level::Initial;
    }
}
