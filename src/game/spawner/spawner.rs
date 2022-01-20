use super::levels::{initial, level1};
use crate::{
    clock::ticker::Ticker,
    game::{
        game_item::GameItem,
        ui::{health::Health, missiles_amount::MissilesAmount, panel::ButtonPanel, score::Score},
    },
    view::viewport::Viewport,
};

pub enum Level {
    Initial,
    Level1,
}

impl Level {
    fn next(&self, _: &Ticker) -> Self {
        match self {
            // TODO Advance levels. Consider using `Countdown` or `Ticker` to time the advancement.
            _ => Self::Level1,
        }
    }

    fn spawn(&self, ticker: &Ticker, viewport: &Viewport) -> Vec<Box<dyn GameItem>> {
        match self {
            Self::Initial => initial(viewport),
            Self::Level1 => level1(ticker, viewport),
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
    pub fn actors(&mut self, ticker: &Ticker, viewport: &Viewport) -> Vec<Box<dyn GameItem>> {
        let actors = self.level.spawn(ticker, viewport);
        self.level = self.level.next(ticker);
        actors
    }

    pub fn ui(&self) -> Vec<Box<dyn GameItem>> {
        vec![
            // Render the health, missiles, and score UIs before the button panel, so that they'll
            // be rendered below the panel when the viewport is very narrow.
            Box::new(Health::default()),
            Box::new(MissilesAmount::default()),
            Box::new(Score::default()),
            Box::new(ButtonPanel::default()),
        ]
    }

    pub fn restart(&mut self) {
        self.level = Level::Initial;
    }
}
