use super::{
    button::ButtonSize,
    container::ButtonContainer,
    factory::{new_game_over, new_missiles, new_rewind, new_shields},
};
use crate::{
    app::command::{Command, CommandHandler, NO_COMMANDS},
    clock::ticker::{TickHandler, Ticker},
    game::game_item::GameItem,
    view::{
        render::Renderable,
        viewport::{Coordinates, Viewport},
    },
};
use tui::widgets::canvas::Context;

const GUTTER_WIDTH: u16 = 1;
const MIN_FULL_WIDTH: u16 = 71;

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
            coordinates: (0, 0), // Will be re-aligned during `render()`
            size: ButtonSize::Full,
        }
    }
}

impl GameItem for ButtonPanel {}

impl Renderable for ButtonPanel {
    fn render(&mut self, context: &mut Context, viewport: &Viewport) {
        self.center(viewport);

        self.size = if viewport.width < MIN_FULL_WIDTH {
            ButtonSize::Condensed
        } else {
            ButtonSize::Full
        };
        let (x, y) = self.coordinates;
        let x_panel_offset = self.width() / 2;
        let x = x.saturating_sub(x_panel_offset);

        for (i, button) in self.buttons.iter_mut().enumerate() {
            let x_buttons_offset = i as u16 * (button.width(self.size) + GUTTER_WIDTH);
            let coordinates = (x + x_buttons_offset, y);
            button.render(context, coordinates, self.size);
        }
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_from_coordinates(self.width(), self.height(), self.coordinates)
    }
}

impl TickHandler for ButtonPanel {
    fn handle_tick(&mut self, ticker: &Ticker) {
        for button in self.buttons.iter_mut() {
            button.handle_tick(ticker)
        }
    }
}

impl ButtonPanel {
    fn center(&mut self, viewport: &Viewport) {
        let (_, y) = self.coordinates;
        let (x, _) = viewport.center();
        self.coordinates = (x, y)
    }

    fn height(&self) -> u16 {
        self.buttons[0].height(self.size) // All buttons are the same height.
    }

    fn width(&self) -> u16 {
        let buttons_width: u16 = self
            .buttons
            .iter()
            .map(|button| button.width(self.size))
            .sum();
        let number_of_gutters = self.buttons.len() as u16 - 1;
        buttons_width + (number_of_gutters * GUTTER_WIDTH)
    }
}
