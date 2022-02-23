use crate::{
    app::{
        color::ColorTheme,
        command::{Command, CommandHandler, NO_COMMANDS},
    },
    clock::ticker::TickHandler,
    game::game_item::GameItem,
    view::{
        coordinates::Coordinates, render::Renderable, renderer::Renderer, util::chars_width,
        viewport::Viewport,
    },
};
use tui::{
    style::{Color, Style},
    text::Span,
};

const HEIGHT: u8 = 1;

static TEXT_HEADER: &str = "Missiles ";
static TEXT_CURRENT: &str = "▮";
static TEXT_USED: &str = "▯";

pub struct MissilesBar {
    coordinates: Coordinates,
    current: u8,
    max: u8,
}

impl CommandHandler for MissilesBar {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        if let Command::UpdateMissiles(current, max) = command {
            self.current = current;
            self.max = max;
        }
        NO_COMMANDS
    }
}
impl Default for MissilesBar {
    fn default() -> Self {
        let coordinates = Coordinates::new(1, 1);
        Self::new(coordinates, 0, 0)
    }
}

impl GameItem for MissilesBar {}

impl Renderable for MissilesBar {
    fn render(&self, renderer: &mut Renderer) {
        let header = span(TEXT_HEADER.to_string(), ColorTheme::MissilesHeader);
        let current = span(self.text_current(), ColorTheme::MissilesCurrent);
        let used = span(self.text_used(), ColorTheme::MissilesLost);
        renderer.render_spans(self.coordinates, vec![header, current, used]);
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_with_coordinates(self.width(), HEIGHT, self.coordinates)
    }
}

impl TickHandler for MissilesBar {}

impl MissilesBar {
    fn new(coordinates: Coordinates, current: u8, max: u8) -> Self {
        Self {
            coordinates,
            current,
            max,
        }
    }

    fn text_current(&self) -> String {
        TEXT_CURRENT.repeat(self.current as usize)
    }

    fn text_used(&self) -> String {
        TEXT_USED.repeat((self.max - self.current) as usize)
    }

    fn width(&self) -> u8 {
        chars_width(TEXT_HEADER) + self.max
    }
}

fn span<'a>(text: String, color: ColorTheme) -> Span<'a> {
    Span::styled(text, Style::default().fg(Color::from(color)))
}
