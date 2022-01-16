use tui::{
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
};

use crate::color::ColorTheme;

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

pub fn create_buttons_block<'a>() -> Block<'a> {
    Block::default().style(Style::default().bg(Color::from(ColorTheme::Bg)))
}

pub fn create_background_block<'a>() -> Block<'a> {
    Block::default().style(Style::default().bg(Color::from(ColorTheme::Bg)))
}
