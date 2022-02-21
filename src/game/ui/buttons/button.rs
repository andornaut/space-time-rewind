use crate::{
    app::{app::TICKS_PER_SECOND, color::ColorTheme, command::Command},
    clock::{countdown::Countdown, ticker::Ticker},
    view::{
        coordinates::Coordinates,
        renderer::Renderer,
        util::{chars_height, chars_width},
    },
};

const ACTIVE_COUNT: u16 = TICKS_PER_SECOND / 10; // 100ms

#[derive(Copy, Clone)]
pub enum ButtonSize {
    Condensed,
    Full,
}

pub trait Button {
    fn color(&self, active: bool, disabled: bool) -> ColorTheme;

    fn handle_command(&mut self, _: Command) -> Option<Command> {
        None
    }

    fn text(&self, size: ButtonSize) -> &'static str;
}

pub struct ButtonContainer {
    active: Countdown,
    button: Box<dyn Button>,
    disabled: Countdown,
}

impl ButtonContainer {
    pub fn new(button: Box<dyn Button>) -> Self {
        Self::new_disableable(button, 0)
    }

    pub fn new_disableable(button: Box<dyn Button>, disabled_count: u16) -> Self {
        Self {
            active: Countdown::new(ACTIVE_COUNT),
            disabled: Countdown::new(disabled_count),
            button,
        }
    }

    pub fn handle_command(&mut self, command: Command) -> Option<Command> {
        if self.disabled.off() {
            if let Some(command) = self.button.handle_command(command) {
                self.active.restart();
                self.disabled.restart();
                return Some(command);
            }
        }
        None
    }

    pub fn handle_tick(&mut self, _: &Ticker) {
        self.active.down();
        self.disabled.down();
    }

    pub fn render(&mut self, renderer: &mut Renderer, coordinates: Coordinates, size: ButtonSize) {
        let color = self.button.color(self.active.on(), self.disabled.on());
        let text = self.button.text(size);
        renderer.render(coordinates, text, color);
    }

    pub fn height(&self, size: ButtonSize) -> u8 {
        chars_height(self.button.text(size))
    }

    pub fn width(&self, size: ButtonSize) -> u8 {
        chars_width(self.button.text(size))
    }
}
