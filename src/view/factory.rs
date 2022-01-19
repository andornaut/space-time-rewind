use super::viewport::Viewport;
use crate::app::color::ColorTheme;
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols::Marker,
    text::Span,
    widgets::{
        canvas::{Canvas, Context},
        Block, BorderType, Borders,
    },
};

const BOARD_MIN_HEIGHT: u16 = 10;
const BUTTON_PANEL_HEIGHT: u16 = 3;
const MAX_HEIGHT: u16 = 40;
const MAX_WIDTH: u16 = 79;
static ACTORS_TITLE: &str = "Space-Time-Rewind!";

pub fn create_actors_block<'a>() -> Block<'a> {
    Block::default()
        .border_style(Style::default().fg(Color::from(ColorTheme::BoardBorderFg)))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::from(ColorTheme::Bg)))
        .title(Span::styled(
            ACTORS_TITLE,
            Style::default().fg(Color::from(ColorTheme::BoardTitleFg)),
        ))
}

pub fn create_ui_block<'a>() -> Block<'a> {
    Block::default().style(Style::default().bg(Color::from(ColorTheme::Bg)))
}

pub fn create_background_block<'a>() -> Block<'a> {
    Block::default().style(Style::default().bg(Color::from(ColorTheme::Bg)))
}

pub fn create_actors_viewport(rect: Rect) -> Viewport {
    let Rect { width, height, .. } = rect;
    // Account for the 1px border
    Viewport::new(width.saturating_sub(2), height.saturating_sub(2))
}

pub fn create_ui_viewport(rect: Rect) -> Viewport {
    let Rect { width, .. } = rect;
    Viewport::new(width, BUTTON_PANEL_HEIGHT)
}

pub fn create_canvas<F>(block: Block, viewport: Viewport) -> Canvas<F>
where
    F: Fn(&mut Context),
{
    let (x_min, y_min) = viewport.bottom_left();
    let (x_max, y_max) = viewport.top_right();
    Canvas::default()
        .background_color(Color::from(ColorTheme::Bg))
        .block(block)
        .marker(Marker::Block)
        .x_bounds([f64::from(x_min), f64::from(x_max)])
        .y_bounds([f64::from(y_min), f64::from(y_max)])
}

pub fn split_into_actors_and_ui(rect: Rect) -> (Rect, Rect) {
    let rect = normalize(rect);
    let constraints = [
        Constraint::Min(BOARD_MIN_HEIGHT),
        Constraint::Length(BUTTON_PANEL_HEIGHT),
    ];
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints.as_ref())
        .split(rect);
    (rects[0], rects[1])
}

fn normalize(rect: Rect) -> Rect {
    let mut normalized_rect = Rect {
        height: MAX_HEIGHT,
        width: MAX_WIDTH,
        ..rect
    }
    .intersection(rect);
    normalized_rect.y += rect.height.saturating_sub(MAX_HEIGHT);
    normalized_rect
}
