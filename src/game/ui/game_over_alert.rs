use crate::{
    app::{color::ColorTheme, command::CommandHandler},
    clock::ticker::TickHandler,
    game::game_item::GameItem,
    view::{
        coordinates::Coordinates,
        render::Renderable,
        renderer::Renderer,
        util::{chars_height, chars_width},
        viewport::Viewport,
    },
};

static TEXT: &str = "\
Game over!   \x20
Press [r] to restart or [q] to quit "; // Add whitespace to the first line to overwrite the health bar

pub struct GameOverAlert {
    coordinates: Coordinates,
    height: u8,
    width: u8,
}

impl GameItem for GameOverAlert {}

impl CommandHandler for GameOverAlert {}

impl Renderable for GameOverAlert {
    fn render(&self, renderer: &mut Renderer) {
        renderer.render(self.viewport(), TEXT, ColorTheme::GameOver);
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_with_coordinates(self.width, self.height, self.coordinates)
    }
}

impl TickHandler for GameOverAlert {}

impl GameOverAlert {
    pub fn new() -> Self {
        let height = chars_height(TEXT);
        let width = chars_width(TEXT);
        Self {
            coordinates: Coordinates::new(1, 1),
            height,
            width,
        }
    }
}
