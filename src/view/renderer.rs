use std::{cell::RefCell, rc::Rc};

use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    symbols,
    text::Span,
    widgets::{
        canvas::{Canvas, Context},
        Block, BorderType, Borders,
    },
    Frame,
};

use crate::{color::ColorTheme, game::GameItem};

use super::viewport::{Coordinates, Viewport};

pub trait Renderable {
    fn render(&mut self, context: &mut Context, viewport: Viewport);

    fn viewport(&self) -> Viewport;
}

pub fn render_text(
    context: &mut Context,
    coordinates: Coordinates,
    text: &'static str,
    color: Color,
) {
    let (x, y) = coordinates;
    // Reverse the string, b/c it is stored top->down, but is rendered bottom->up.
    for (y_offset, line) in text.lines().rev().enumerate() {
        context.print(f64::from(x), f64::from(y) + y_offset as f64, line, color);
    }
}

// TODO figure how to make this take Renderable instead of GameItem.
pub fn render_board<B: Backend>(
    frame: &mut Frame<B>,
    renderables: &mut Vec<Box<dyn GameItem>>,
    rect: Rect,
) {
    let block = create_board_block();

    // `viewport` should use relative coordinates, eg. (0, 0)
    let Rect { width, height, .. } = rect;

    // The `viewport` is a bit smaller than the `rect`, b/c we apply a border above.
    let viewport = Viewport::new(width - 2, height - 2);
    render_canvas(frame, renderables, rect, block, viewport);
}

pub fn render_buttons<B: Backend>(
    frame: &mut Frame<B>,
    renderables: &mut Vec<Box<dyn GameItem>>,
    rect: Rect,
) {
    let block = create_buttons_block();

    // `viewport` should use relative coordinates, eg. (0, 0)
    let Rect { width, height, .. } = rect;
    let viewport = Viewport::new(width, height);
    render_canvas(frame, renderables, rect, block, viewport);
}

fn render_canvas<B: Backend>(
    frame: &mut Frame<B>,
    renderables: &mut Vec<Box<dyn GameItem>>,
    rect: Rect,
    block: Block,
    viewport: Viewport,
) {
    let (x_max, y_max) = viewport.top_right();
    let mut canvas = Canvas::default()
        .background_color(Color::from(ColorTheme::Bg))
        .block(block)
        .marker(symbols::Marker::Block)
        .x_bounds([0.0, f64::from(x_max)])
        .y_bounds([0.0, f64::from(y_max)]);
    let renderables = Rc::new(RefCell::new(renderables));
    canvas = canvas.paint(|ctx| {
        for renderable in renderables.borrow_mut().iter_mut() {
            renderable.render(ctx, viewport);
        }
    });
    frame.render_widget(canvas, rect);
}

fn create_board_block<'a>() -> Block<'a> {
    static BOARD_TITLE: &str = "Space-Time-Rewind!";
    Block::default()
        .border_style(Style::default().fg(Color::from(ColorTheme::BoardBorderFg)))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::from(ColorTheme::Bg)))
        .title(Span::styled(
            BOARD_TITLE,
            Style::default().fg(Color::from(ColorTheme::BoardTitleFg)),
        ))
}

fn create_buttons_block<'a>() -> Block<'a> {
    Block::default().style(Style::default().bg(Color::from(ColorTheme::Bg)))
}
