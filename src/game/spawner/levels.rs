use crate::{
    clock::ticker::{Frequency, Ticker},
    game::{
        actors::{asteroid::Asteroid, ship::Ship},
        GameItem,
    },
    view::viewport::Viewport,
};

pub fn initial(viewport: &Viewport) -> Vec<Box<dyn GameItem>> {
    let (x, y) = viewport.top_right();
    vec![
        Box::new(Ship::new(viewport.center())),
        Box::new(Asteroid::new((0, y))),
        Box::new(Asteroid::new((x / 5, y + 3))),
        Box::new(Asteroid::new((x / 3, y))),
        Box::new(Asteroid::new((x.saturating_sub(5), y))),
    ]
}

pub fn level1(ticker: &Ticker, viewport: &Viewport) -> Vec<Box<dyn GameItem>> {
    let mut actors: Vec<Box<dyn GameItem>> = Vec::new();
    let (x, y) = viewport.top_right();
    if ticker.should(Frequency::Ten) {
        actors.push(Box::new(Asteroid::new((0, y))));
        actors.push(Box::new(Asteroid::new(((x / 2) + 3, y + 6))));
        actors.push(Box::new(Asteroid::new((x.saturating_sub(10), y))));
    }
    actors
}
