use super::button::{ButtonContainer, ButtonSize};
use crate::{
    app::command::{Command, CommandHandler, NO_COMMANDS},
    clock::ticker::{TickHandler, Ticker},
    game::game_item::GameItem,
    view::{coordinates::Coordinates, render::Renderable, renderer::Renderer, viewport::Viewport},
};

const GUTTER_WIDTH: u8 = 1;
const MIN_FULL_WIDTH: u8 = 58;

pub struct ButtonPanel {
    buttons: Vec<ButtonContainer>,
    coordinates: Coordinates,
    size: ButtonSize,
}

impl CommandHandler for ButtonPanel {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        match command {
            Command::GameOver => {
                self.buttons = vec![ButtonContainer::new_game_over()];
                // Align to the top-left of the UI viewport.
                self.coordinates =
                    Coordinates::new(GUTTER_WIDTH, i8::try_from(GUTTER_WIDTH).unwrap());
                return NO_COMMANDS;
            }
            Command::UiViewportInitializedOrChanged(viewport) => {
                self.align(viewport);
                return NO_COMMANDS;
            }
            _ => (),
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
            buttons: vec![
                ButtonContainer::new_missiles(),
                ButtonContainer::new_shields(),
            ],
            coordinates: Coordinates::default(), // Will be re-aligned during `render()`
            size: ButtonSize::Full,
        }
    }
}

impl GameItem for ButtonPanel {}

impl Renderable for ButtonPanel {
    // The lint warning is a false positive: https://github.com/rust-lang/rust-clippy/issues/7414
    #[allow(clippy::clone_on_copy)]
    fn render(&self, renderer: &mut Renderer) {
        for (i, button) in self.buttons.iter().enumerate() {
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
    fn handle_tick(&mut self, ticker: &Ticker, _: Viewport) {
        for button in self.buttons.iter_mut() {
            button.handle_tick(ticker)
        }
    }
}

impl ButtonPanel {
    fn align(&mut self, viewport: Viewport) {
        self.size = if viewport.width() < MIN_FULL_WIDTH {
            ButtonSize::Condensed
        } else {
            ButtonSize::Full
        };

        let (x_centered, _) = viewport.centered().as_tuple();
        let x = x_centered.saturating_sub(self.width() / 2); // Avoid a negative x-position when the viewport is very narrow.
        self.coordinates = Coordinates::new(x, 0);
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
