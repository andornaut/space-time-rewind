use crate::{
    clock::ticker::{Frequency, Ticker},
    game::{actors::asteroid::Asteroid, buttons::panel::ButtonPanel},
};

use super::{actors::ship::Ship, GameItem};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ActorSpawns {
    Initial,
    Level1,
}

impl ActorSpawns {
    fn next(self) -> Self {
        match self {
            // TODO Advance levels. Consider using `Countdown` or `Ticker` to time the advancement.
            _ => Self::Level1,
        }
    }

    fn spawn(self, ticker: &Ticker) -> Vec<Box<dyn GameItem>> {
        match self {
            Self::Initial => Self::initial_actors(),
            Self::Level1 => Self::level1_actors(ticker),
        }
    }

    fn initial_actors() -> Vec<Box<dyn GameItem>> {
        vec![
            Box::new(Ship::default()),
            Box::new(Asteroid::new((0, 50))),
            Box::new(Asteroid::new((10, 50))),
            Box::new(Asteroid::new((20, 45))),
            Box::new(Asteroid::new((45, 50))),
        ]
    }

    fn level1_actors(ticker: &Ticker) -> Vec<Box<dyn GameItem>> {
        let mut actors: Vec<Box<dyn GameItem>> = Vec::new();
        if ticker.should(Frequency::Ten) {
            actors.push(Box::new(Asteroid::new((0, 50))));
            actors.push(Box::new(Asteroid::new((15, 45))));
            actors.push(Box::new(Asteroid::new((45, 55))));
        }
        return actors;
    }
}

pub struct Spawner {
    state: ActorSpawns,
}

impl Default for Spawner {
    fn default() -> Self {
        Spawner {
            state: ActorSpawns::Initial,
        }
    }
}

impl Spawner {
    pub fn actors(&self, ticker: &Ticker) -> Vec<Box<dyn GameItem>> {
        self.state.spawn(ticker)
    }

    pub fn buttons(&self) -> Vec<Box<dyn GameItem>> {
        vec![Box::new(ButtonPanel::default())]
    }

    pub fn next(&mut self) {
        self.state = self.state.next();
    }

    pub fn restart(&mut self) {
        self.state = ActorSpawns::Initial;
    }
}
