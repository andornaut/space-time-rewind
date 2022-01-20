use super::viewport::Viewport;
use crate::app::color::ColorTheme;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols::Marker,
    text::Span,
    widgets::{
        canvas::{Canvas, Context},
        Block, BorderType, Borders, Paragraph, Wrap,
    },
};

const ACTORS_MIN_HEIGHT: u16 = 10;
const UI_HEIGHT: u16 = 3;
pub const WINDOW_MIN_HEIGHT: u16 = ACTORS_MIN_HEIGHT + UI_HEIGHT;
pub const WINDOW_MIN_WIDTH: u16 = 47;

const MAX_HEIGHT: u16 = 40;
const MAX_WIDTH: u16 = 79;
static TITLE: &str = "Space-Time-Rewind!";
static ERROR_MESSAGE_RESIZE: &str = "Please increase the size of the terminal window";

pub fn create_actors_block<'a>() -> Block<'a> {
    Block::default()
        .border_style(Style::default().fg(Color::from(ColorTheme::BoardBorderFg)))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::from(ColorTheme::Bg)))
        .title(Span::styled(
            TITLE,
            Style::default().fg(Color::from(ColorTheme::BoardTitleFg)),
        ))
}

pub fn create_error_message<'a>() -> Paragraph<'a> {
    let block = Block::default()
        .border_style(Style::default().fg(Color::from(ColorTheme::ErrorBg)))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::from(ColorTheme::Bg)))
        .title(Span::styled(
            TITLE,
            Style::default().fg(Color::from(ColorTheme::ErrorBg)),
        ));
    Paragraph::new(ERROR_MESSAGE_RESIZE)
        .style(
            Style::default()
                .bg(Color::from(ColorTheme::ErrorBg))
                .fg(Color::from(ColorTheme::ErrorFg)),
        )
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
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
    Viewport::new(width, UI_HEIGHT)
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
        Constraint::Min(ACTORS_MIN_HEIGHT),
        Constraint::Length(UI_HEIGHT),
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
