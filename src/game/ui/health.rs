use crate::{
    app::{
        color::ColorTheme,
        command::{Command, CommandHandler, NO_COMMANDS},
    },
    clock::ticker::TickHandler,
    game::{game_item::GameItem, INITIAL_MAX_HEALTH},
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
static TEXT_HEADER: &'static str = "Health ";
static TEXT_HEALTH_CURRENT: &'static str = "▮";
static TEXT_HEALTH_LOST: &'static str = "▯";

pub struct Health {
    coordinates: Coordinates,
    current: u8,
    max: u8,
}

impl CommandHandler for Health {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        match command {
            Command::Health(current, max) => {
                self.current = current;
                self.max = max;
            }
            _ => (),
        }
        NO_COMMANDS
    }
}
impl Default for Health {
    fn default() -> Self {
        Self::new((1, 1), INITIAL_MAX_HEALTH, INITIAL_MAX_HEALTH)
    }
}

impl GameItem for Health {}

impl Renderable for Health {
    fn render(&mut self, context: &mut Context, _: &Viewport) {
        let header = span(TEXT_HEADER.to_string(), ColorTheme::HealthHeader);
        let current = span(self.text_current(), ColorTheme::HealthCurrent);
        let lost = span(self.text_lost(), ColorTheme::HealthLost);
        let (x, y) = self.coordinates;
        context.print(
            f64::from(x),
            f64::from(y),
            Spans::from(vec![header, current, lost]),
        );
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_from_coordinates(self.width(), HEIGHT, self.coordinates)
    }
}

impl TickHandler for Health {}

impl Health {
    fn new(coordinates: Coordinates, current: u8, max: u8) -> Self {
        Self {
            coordinates,
            current,
            max,
        }
    }

    fn text_current(&self) -> String {
        TEXT_HEALTH_CURRENT.repeat(self.current as usize)
    }

    fn text_lost(&self) -> String {
        TEXT_HEALTH_LOST.repeat((self.max - self.current) as usize)
    }

    fn width(&self) -> u16 {
        chars_width(TEXT_HEADER)
            + chars_width(self.text_current().as_str())
            + chars_width(self.text_lost().as_str())
    }
}

fn span<'a>(text: String, color: ColorTheme) -> Span<'a> {
    Span::styled(text, Style::default().fg(Color::from(color)))
}
