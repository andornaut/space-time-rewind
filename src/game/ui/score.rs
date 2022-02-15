use std::cmp::max;

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

const GUTTER_HEIGHT: u8 = 1;
const GUTTER_WIDTH: u8 = 1;
const HEIGHT: u8 = 2;
static TEXT_HEADER: &str = "Score";

pub struct Score {
    coordinates: Coordinates,
    score: u32,
}

impl CommandHandler for Score {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        if let Command::IncreaseScore(number) = command {
            self.score += u32::from(number);
        }
        NO_COMMANDS
    }
}
impl Default for Score {
    fn default() -> Self {
        Self::new(Coordinates::default()) // Will be re-aligned during `render()`
    }
}

impl GameItem for Score {}

impl Renderable for Score {
    fn render(&mut self, renderer: &mut Renderer, visible_viewport: &Viewport) {
        let width = self.width();
        // One of these offsets will be 0.
        let header_offset = width - chars_width(TEXT_HEADER);
        let points_offset = width - chars_width(self.text().as_str());

        let header_spans = vec![Span::styled(
            TEXT_HEADER,
            Style::default().fg(Color::from(ColorTheme::ScoreHeader)),
        )];
        let points_spans = vec![Span::styled(
            self.text(),
            Style::default().fg(Color::from(ColorTheme::ScorePoints)),
        )];

        self.align(visible_viewport); // Must `align()` before accessing `self.coordinates`
        let (x, y) = self.coordinates.as_tuple();
        renderer.render_spans(Coordinates::new(x + points_offset, y), points_spans);
        renderer.render_spans(Coordinates::new(x + header_offset, y + 1), header_spans);
    }

    fn viewport(&self) -> Viewport {
        Viewport::new_with_coordinates(self.width(), HEIGHT, self.coordinates)
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

    fn align(&mut self, viewport: &Viewport) {
        let (x, _) = viewport.top_right();
        let (_, y) = viewport.bottom_left();
        let x = x - self.width() - GUTTER_WIDTH;
        let y = y + i8::try_from(GUTTER_HEIGHT).unwrap();
        self.coordinates = Coordinates::new(x, y);
    }

    fn text(&self) -> String {
        self.score.to_string()
    }

    fn width(&self) -> u8 {
        max(chars_width(TEXT_HEADER), chars_width(self.text().as_str()))
    }
}
