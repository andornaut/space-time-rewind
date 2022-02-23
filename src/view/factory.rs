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

const ACTORS_MIN_HEIGHT: u8 = 7;
const UI_HEIGHT: u8 = 3;

const MAX_HEIGHT: u8 = 40;
const MAX_WIDTH: u8 = 100;
pub const WINDOW_MIN_HEIGHT: u8 = ACTORS_MIN_HEIGHT + UI_HEIGHT;
pub const WINDOW_MIN_WIDTH: u8 = 19;
pub const WORLD_HEIGHT: u8 = MAX_HEIGHT - UI_HEIGHT - 2; // Account for the actors viewport's borders
pub const WORLD_WIDTH: u8 = 200;

static TITLE: &str = "Space-Time-Rewind!";
static RESIZE_WARNING_MESSAGE: &str = "Please increase the size of the terminal window";

pub fn create_actors_block<'a>() -> Block<'a> {
    let title = create_title(ColorTheme::BoardTitleFg);
    with_default_borders(create_background_block()).title(title)
}

pub fn create_background_block<'a>() -> Block<'a> {
    Block::default().style(Style::default().bg(Color::from(ColorTheme::Bg)))
}

pub fn create_resize_warning_paragraph<'a>() -> Paragraph<'a> {
    let title = create_title(ColorTheme::ErrorFg);
    let block = with_error_borders(create_background_block()).title(title);
    Paragraph::new(RESIZE_WARNING_MESSAGE)
        .style(
            Style::default()
                .bg(Color::from(ColorTheme::Bg))
                .fg(Color::from(ColorTheme::ErrorFg)),
        )
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

pub fn create_ui_block<'a>() -> Block<'a> {
    Block::default().style(Style::default().bg(Color::from(ColorTheme::Bg)))
}

pub fn create_actors_viewport(rect: Rect) -> Viewport {
    let Rect { width, height, .. } = rect;
    // Account for the 1px border
    let width = u8::try_from(width.saturating_sub(2)).unwrap();
    let height = u8::try_from(height.saturating_sub(2)).unwrap();
    Viewport::new(width, height)
}

pub fn create_ui_viewport(rect: Rect) -> Viewport {
    let Rect { width, .. } = rect;
    let width = u8::try_from(width).unwrap();
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
        Constraint::Min(u16::from(ACTORS_MIN_HEIGHT)),
        Constraint::Length(u16::from(UI_HEIGHT)),
    ];
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints.as_ref())
        .split(rect);
    (rects[0], rects[1])
}

fn create_title<'a>(color: ColorTheme) -> Span<'a> {
    Span::styled(TITLE, Style::default().fg(Color::from(color)))
}

fn normalize(rect: Rect) -> Rect {
    let max_height = u16::from(MAX_HEIGHT);
    let max_width = u16::from(MAX_WIDTH);
    let mut normalized_rect = Rect {
        height: max_height,
        width: max_width,
        ..rect
    }
    .intersection(rect);
    // Add top-padding
    normalized_rect.y += rect.height.saturating_sub(max_height);
    normalized_rect
}

fn with_default_borders(block: Block) -> Block {
    block
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::from(ColorTheme::BoardBorderFg)))
        .border_type(BorderType::Rounded)
}

fn with_error_borders(block: Block) -> Block {
    block
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::from(ColorTheme::ErrorFg)))
        .border_type(BorderType::Rounded)
}
