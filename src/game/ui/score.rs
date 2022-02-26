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

const MARGIN_LENGTH: u8 = 1;
const HEIGHT: u8 = 2;
static TEXT_HEADER: &str = "Score";

pub struct Score {
    coordinates: Coordinates,
    score: u32,
}

impl CommandHandler for Score {
    fn handle_command(&mut self, command: Command) -> Vec<Command> {
        match command {
            Command::IncreaseScore(number) => {
                self.score += number;
            }
            Command::UiViewportInitializedOrChanged(viewport) => {
                self.align(viewport);
            }
            _ => (),
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
    fn render(&self, renderer: &mut Renderer) {
        let (x, y) = self.coordinates.as_tuple();

        let width = self.width();
        let header_offset = width - chars_width(TEXT_HEADER);
        let header_spans = vec![Span::styled(
            TEXT_HEADER,
            Style::default().fg(Color::from(ColorTheme::ScoreHeader)),
        )];
        let header_coordinates = Coordinates::new(x + header_offset, y + 1);

        let points_offset = width - chars_width(self.text().as_str());
        let points_spans = vec![Span::styled(
            self.text(),
            Style::default().fg(Color::from(ColorTheme::ScorePoints)),
        )];
        let points_coordinates = Coordinates::new(x + points_offset, y);

        let viewport = self.viewport();
        renderer.render_spans(viewport.with_coordinates(points_coordinates), points_spans);
        renderer.render_spans(viewport.with_coordinates(header_coordinates), header_spans);
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

    fn align(&mut self, viewport: Viewport) {
        let (x, _) = viewport.top_right().as_tuple();
        let (_, y) = viewport.bottom_left().as_tuple();
        let x = x - self.width() + 1 - MARGIN_LENGTH;
        let y = y + i8::try_from(MARGIN_LENGTH).unwrap();
        self.coordinates = Coordinates::new(x, y);
    }

    fn text(&self) -> String {
        self.score.to_string()
    }

    fn width(&self) -> u8 {
        max(chars_width(TEXT_HEADER), chars_width(self.text().as_str()))
    }
}
