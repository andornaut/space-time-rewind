use crate::{
    app::{
        color::ColorTheme,
        command::{Command, CommandHandler, NO_COMMANDS},
    },
    clock::ticker::TickHandler,
    game::{game_item::GameItem, INITIAL_MAX_MISSILES},
    view::{
        render::Renderable,
        util::chars_width,
        viewport::{Coordinates, Viewport},
    },
};
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::canvas::Context,
};

const HEIGHT: u16 = 1;
static TEXT_HEADER: &'static str = "Missiles ";
static TEXT_CURRENT: &'static str = "▮";
static TEXT_USED: &'static str = "▯";

pub struct MissilesAmount {
    coordinates: Coordinates,
    current: u8,
    max: u8,
}

impl CommandHandler for MissilesAmount {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        match command {
            Command::UpdateMissiles(current, max) => {
                self.current = current;
                self.max = max;
            }
            _ => (),
        }
        NO_COMMANDS
    }
}
impl Default for MissilesAmount {
    fn default() -> Self {
        Self::new((1, 1), INITIAL_MAX_MISSILES, INITIAL_MAX_MISSILES)
    }
}

impl GameItem for MissilesAmount {}

impl Renderable for MissilesAmount {
    fn render(&mut self, context: &mut Context, _: &Viewport) {
        let header = span(TEXT_HEADER.to_string(), ColorTheme::MissilesHeader);
        let current = span(self.text_current(), ColorTheme::MissilesCurrent);
        let used = span(self.text_used(), ColorTheme::MissilesLost);
        let (x, y) = self.coordinates;
        context.print(
            f64::from(x),
            f64::from(y),
            Spans::from(vec![header, current, used]),
        );
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_from_coordinates(self.width(), HEIGHT, self.coordinates)
    }
}

impl TickHandler for MissilesAmount {}

impl MissilesAmount {
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

    fn width(&self) -> u16 {
        chars_width(TEXT_HEADER) + u16::from(self.max)
    }
}

fn span<'a>(text: String, color: ColorTheme) -> Span<'a> {
    Span::styled(text, Style::default().fg(Color::from(color)))
}
