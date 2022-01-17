use super::viewport::{Coordinates, Viewport};
use crate::{app::color::ColorTheme, game::game_item::GameItem};
use std::{cell::RefCell, rc::Rc};
use tui::{
    backend::Backend,
    layout::Rect,
    style::Color,
    symbols,
    widgets::{
        canvas::{Canvas, Context},
        Block,
    },
    Frame,
};

const MIN_CANVAS_LENGTH: u16 = 3;

pub trait Renderable {
    fn render(&mut self, context: &mut Context, viewport: Viewport);

    fn viewport(&self) -> Viewport;
}

pub fn render_canvas<B: Backend>(
    frame: &mut Frame<B>,
    renderables: &mut Vec<Box<dyn GameItem>>,
    block: Block,
    rect: Rect,
    viewport: Viewport,
) {
    if rect.width < MIN_CANVAS_LENGTH || rect.height < MIN_CANVAS_LENGTH {
        // Skip rendering the canvas to avoid a panic when `rect` is too small.
        return;
    }
    let (x_min, y_min) = viewport.bottom_left();
    let (x_max, y_max) = viewport.top_right();
    let mut canvas = Canvas::default()
        .background_color(Color::from(ColorTheme::Bg))
        .block(block)
        .marker(symbols::Marker::Block)
        // The scaling doesn't work correctly, unless each bounds is 1 less than the top_right position.
        .x_bounds([f64::from(x_min), f64::from(x_max - 1)])
        .y_bounds([f64::from(y_min), f64::from(y_max - 1)]);

    let renderables = Rc::new(RefCell::new(renderables));
    canvas = canvas.paint(|ctx| {
        for renderable in renderables.borrow_mut().iter_mut() {
            renderable.render(ctx, viewport);
        }
    });
    frame.render_widget(canvas, rect);
}

pub fn render_text(
    context: &mut Context,
    coordinates: Coordinates,
    text: &'static str,
    color: Color,
) {
    let (x, y) = coordinates;
    // Reverse the string, because it is stored top->down, but is rendered bottom->up.
    for (y_offset, line) in text.lines().rev().enumerate() {
        context.print(f64::from(x), f64::from(y) + y_offset as f64, line, color);
    }
}
