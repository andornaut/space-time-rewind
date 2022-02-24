use crate::{
    clock::ticker::{Frequency, Ticker},
    game::{
        actors::{asteroid::Asteroid, power_up::PowerUp, ship::Ship},
        game_item::GameItem,
    },
    view::{coordinates::Coordinates, viewport::Viewport},
};

pub fn initial(viewport: Viewport) -> Vec<Box<dyn GameItem>> {
    let (_, y) = viewport.top_right().as_tuple();

    let mut actors: Vec<Box<dyn GameItem>> = vec![
        Box::new(Ship::new(Coordinates::default())), // The ship will center itself when first rendered.
        Box::new(Asteroid::new_small(Coordinates::new(1, y - 6))),
        Box::new(Asteroid::new_large(Coordinates::new(15, y))),
    ];
    for x in (0..viewport.width()).step_by(25) {
        actors.push(Box::new(Asteroid::new_medium(Coordinates::new(x, y))));
    }
    actors
}

pub fn level1(ticker: &Ticker, viewport: Viewport) -> Vec<Box<dyn GameItem>> {
    let mut actors: Vec<Box<dyn GameItem>> = Vec::new();

    if ticker.at(Frequency::Eight) {
        let (_, y) = viewport.top_right().as_tuple();

        actors.push(Box::new(Asteroid::new_small(Coordinates::new(0, y))));
        actors.push(Box::new(Asteroid::new_large(Coordinates::new(0, y - 4))));
        actors.push(Box::new(PowerUp::new_health(Coordinates::new(
            (38) - 3,
            y - 1,
        ))));
        actors.push(Box::new(PowerUp::new_missile(Coordinates::new(
            (37) - 3,
            y - 5,
        ))));
        for x in (0..viewport.width()).step_by(25) {
            actors.push(Box::new(Asteroid::new_medium(Coordinates::new(x, y))));
        }
    }
    actors
}
