use super::button::Button;
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

pub struct ButtonPanel {
    buttons: Vec<Button>,
    coordinates: Coordinates,
}

impl CommandHandler for ButtonPanel {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        if let Command::GameOver = command {
            self.buttons.clear();
            self.buttons.push(Button::new_game_over());
            return NO_COMMANDS;
        }
        self.buttons
            .iter_mut()
            .flat_map(|button| button.handle_command(command))
            .collect()
    }
}

impl Default for ButtonPanel {
    fn default() -> Self {
        Self {
            buttons: vec![
                Button::new_missile(),
                Button::new_shields(),
                Button::new_rewind(),
            ],
            coordinates: (0, 0), // Will be re-aligned during `render()`
        }
    }
}

impl GameItem for ButtonPanel {}

impl Renderable for ButtonPanel {
    fn render(&mut self, context: &mut Context, viewport: Viewport) {
        self.center(viewport);
        self.align_buttons();

        for button in self.buttons.iter_mut() {
            button.render(context, viewport);
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
    fn align_buttons(&mut self) {
        let (x, y) = self.coordinates;
        let x_panel_offset = self.width() / 2;
        let x = x.saturating_sub(x_panel_offset);

        for (i, button) in self.buttons.iter_mut().enumerate() {
            let x_buttons_offset = i as u16 * (button.width() + GUTTER_WIDTH);
            button.set_coordinates((x + x_buttons_offset, y));
        }
    }

    fn center(&mut self, viewport: Viewport) {
        let (_, y) = self.coordinates;
        let (x, _) = viewport.center();
        self.coordinates = (x, y)
    }

    fn height(&self) -> u16 {
        self.buttons[0].height() // All buttons are the same height.
    }

    fn width(&self) -> u16 {
        let buttons_width: u16 = self.buttons.iter().map(|button| button.width()).sum();
        let number_of_gutters = self.buttons.len() as u16 - 1;
        buttons_width + (number_of_gutters * GUTTER_WIDTH)
    }
}
