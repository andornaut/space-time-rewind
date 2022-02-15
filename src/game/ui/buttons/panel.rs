use super::{
    button::ButtonSize,
    container::ButtonContainer,
    factory::{new_game_over, new_missiles, new_rewind, new_shields},
};
use crate::{
    app::command::{Command, CommandHandler, NO_COMMANDS},
    clock::ticker::{TickHandler, Ticker},
    game::game_item::GameItem,
    view::{coordinates::Coordinates, render::Renderable, renderer::Renderer, viewport::Viewport},
};

const GUTTER_WIDTH: u8 = 1;
const MIN_FULL_WIDTH: u8 = 71;

pub struct ButtonPanel {
    buttons: Vec<ButtonContainer>,
    coordinates: Coordinates,
    size: ButtonSize,
}

impl CommandHandler for ButtonPanel {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        if let Command::GameOver = command {
            self.buttons = vec![new_game_over()];
            return NO_COMMANDS;
        }
        self.buttons
            .iter_mut()
            .filter_map(|button| button.handle_command(command))
            .collect()
    }
}

impl Default for ButtonPanel {
    fn default() -> Self {
        Self {
            buttons: vec![new_missiles(), new_shields(), new_rewind()],
            coordinates: Coordinates::default(), // Will be re-aligned during `render()`
            size: ButtonSize::Full,
        }
    }
}

impl GameItem for ButtonPanel {}

impl Renderable for ButtonPanel {
    // The lint warning is a false positive: https://github.com/rust-lang/rust-clippy/issues/7414
    #[allow(clippy::clone_on_copy)]
    fn render(&mut self, renderer: &mut Renderer, visible_viewport: &Viewport) {
        self.align(visible_viewport);
        self.resize(visible_viewport);

        for (i, button) in self.buttons.iter_mut().enumerate() {
            let x = u8::try_from(i).unwrap() * (button.width(self.size) + GUTTER_WIDTH);

            let mut coordinates = self.coordinates.clone();
            coordinates.x_offset(i16::from(x));
            button.render(renderer, coordinates, self.size);
        }
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_with_coordinates(self.width(), self.height(), self.coordinates)
    }
}

impl TickHandler for ButtonPanel {
    fn handle_tick(&mut self, ticker: &Ticker, _: &Viewport) {
        for button in self.buttons.iter_mut() {
            button.handle_tick(ticker)
        }
    }
}

impl ButtonPanel {
    fn align(&mut self, viewport: &Viewport) {
        let (x_centered, _) = viewport.centered().as_tuple();
        let x_offset = self.width() / 2;
        self.coordinates = Coordinates::new(x_centered - x_offset, 0);
    }

    fn resize(&mut self, viewport: &Viewport) {
        self.size = if viewport.width() < MIN_FULL_WIDTH {
            ButtonSize::Condensed
        } else {
            ButtonSize::Full
        };
    }

    fn height(&self) -> u8 {
        self.buttons[0].height(self.size) // All buttons are the same height.
    }

    fn width(&self) -> u8 {
        let buttons_width: u8 = self
            .buttons
            .iter()
            .map(|button| button.width(self.size))
            .sum();
        let number_of_gutters = u8::try_from(self.buttons.len()).unwrap() - 1;
        buttons_width + (number_of_gutters * GUTTER_WIDTH)
    }
}
