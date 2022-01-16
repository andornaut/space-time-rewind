use super::levels::{initial, level1};
use crate::{
    clock::ticker::Ticker,
    game::{buttons::panel::ButtonPanel, GameItem},
    view::viewport::Viewport,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LevelState {
    Initial,
    Level1,
}

impl LevelState {
    fn next(self, _: &Ticker) -> Self {
        match self {
            // TODO Advance levels. Consider using `Countdown` or `Ticker` to time the advancement.
            _ => Self::Level1,
        }
    }

    fn spawn(self, ticker: &Ticker, viewport: &Viewport) -> Vec<Box<dyn GameItem>> {
        match self {
            Self::Initial => initial(viewport),
            Self::Level1 => level1(ticker, viewport),
        }
    }
}

pub struct Spawner {
    level: LevelState,
}

impl Default for Spawner {
    fn default() -> Self {
        Spawner {
            level: LevelState::Initial,
        }
    }
}

impl Spawner {
    pub fn actors(&mut self, ticker: &Ticker, viewport: &Viewport) -> Vec<Box<dyn GameItem>> {
        let actors = self.level.spawn(ticker, viewport);
        self.level = self.level.next(ticker);
        actors
    }

    pub fn buttons(&self) -> Vec<Box<dyn GameItem>> {
        vec![Box::new(ButtonPanel::default())]
    }

    pub fn restart(&mut self) {
        self.level = LevelState::Initial;
    }
}
