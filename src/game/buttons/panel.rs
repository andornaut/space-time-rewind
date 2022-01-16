use super::button::Button;
use crate::{
    clock::ticker::{TickHandler, Ticker},
    command::{Command, CommandHandler},
    game::game_item::{GameItem, GameItemKind},
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
    fn handle_command(&mut self, command: Command) -> Command {
        if let Command::GameOver = command {
            self.buttons.clear();
            self.buttons.push(Button::new_game_over());
            return Command::NOOP;
        }

        for button in self.buttons.iter_mut() {
            let secondary_command = button.handle_command(command);
            if secondary_command != Command::NOOP {
                // Only one `Button` should respond to a `command`, so we can short circuit here.
                return secondary_command;
            }
        }
        Command::NOOP
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
            coordinates: (0, 0), // Will center during render()
        }
    }
}

impl GameItem for ButtonPanel {
    fn kind(&self) -> GameItemKind {
        GameItemKind::Button
    }
}

impl Renderable for ButtonPanel {
    fn render(&mut self, context: &mut Context, viewport: Viewport) {
        self.center_buttons(viewport);

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
    fn bottom_left_centered(&self, viewport: Viewport) -> Coordinates {
        let (x, y) = self.coordinates;
        let (x_centered, _) = viewport.center();
        let x_panel_offset = self.width() / 2;
        let x = x.saturating_add(x_centered).saturating_sub(x_panel_offset);
        (x, y)
    }

    fn center_buttons(&mut self, viewport: Viewport) {
        let (x, y) = self.bottom_left_centered(viewport);
        for (i, button) in self.buttons.iter_mut().enumerate() {
            let coordinates = (x + (i as u16 * (button.width() + GUTTER_WIDTH)), y);
            button.set_coordinates(coordinates);
        }
    }

    fn height(&self) -> u16 {
        self.buttons[0].height()
    }

    fn width(&self) -> u16 {
        let buttons_width: u16 = self.buttons.iter().map(|button| button.width()).sum();
        let number_of_gutters = self.buttons.len() as u16 - 1;
        buttons_width + (number_of_gutters * GUTTER_WIDTH)
    }
}
