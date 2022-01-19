use std::cmp::max;

use crate::{
    app::{
        color::ColorTheme,
        command::{Command, CommandHandler, NO_COMMANDS},
    },
    clock::ticker::TickHandler,
    game::game_item::GameItem,
    view::{
        render::Renderable,
        util::chars_width,
        viewport::{Coordinates, Viewport},
    },
};
use tui::{
    style::{Color, Style},
    text::Span,
    widgets::canvas::Context,
};

const GUTTER_WIDTH: u16 = 1;
const HEIGHT: u16 = 2;
static TEXT_HEADER: &'static str = "Score";

#[derive(Clone, Debug)]
pub struct Score {
    coordinates: Coordinates,
    score: u32,
}

impl CommandHandler for Score {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        match command {
            Command::IncreaseScore(number) => self.score += u32::from(number),
            _ => (),
        }
        NO_COMMANDS
    }
}
impl Default for Score {
    fn default() -> Self {
        Self::new((0, 0)) // Will be re-aligned during `render()`
    }
}

impl GameItem for Score {}

impl Renderable for Score {
    fn render(&mut self, context: &mut Context, viewport: Viewport) {
        let width = self.width();
        // One of these offsets will be 0.
        let header_offset = width - chars_width(TEXT_HEADER);
        let points_offset = width - chars_width(self.text().as_str());

        let header_span = Span::styled(
            TEXT_HEADER,
            Style::default().fg(Color::from(ColorTheme::ScoreHeader)),
        );
        let points_span = Span::styled(
            self.text(),
            Style::default().fg(Color::from(ColorTheme::ScorePoints)),
        );

        self.align(viewport); // Must `align()` before accessing `self.coordinates`
        let (x, y) = self.coordinates;
        context.print(f64::from(x + header_offset), f64::from(y + 1), header_span);
        context.print(f64::from(x + points_offset), f64::from(y), points_span);
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_from_coordinates(self.width(), HEIGHT, self.coordinates)
    }
}

impl TickHandler for Score {}

impl Score {
    fn new(coordinates: Coordinates) -> Self {
        Self {
            coordinates,
            score: 0,
        }
    }

    fn align(&mut self, viewport: Viewport) {
        let (x, _) = viewport.top_right();
        let (_, y) = viewport.bottom_left();
        self.coordinates = (
            x.saturating_sub(self.width()) - GUTTER_WIDTH,
            y + GUTTER_WIDTH,
        );
    }

    fn text(&self) -> String {
        self.score.to_string()
    }

    fn width(&self) -> u16 {
        max(chars_width(TEXT_HEADER), chars_width(self.text().as_str()))
    }
}
