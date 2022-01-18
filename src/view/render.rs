use super::{
    factory::create_canvas,
    viewport::{Coordinates, Viewport},
};
use crate::{
    game::{game_item::GameItem, world::World},
    view::{
        factory::{
            create_actors_block, create_actors_viewport, create_background_block, create_ui_block,
            create_ui_viewport, split_into_actors_and_ui,
        },
        session::Session,
    },
};
use anyhow::Result;
use std::{cell::RefCell, rc::Rc};
use tui::{
    backend::Backend,
    layout::Rect,
    style::Color,
    widgets::{canvas::Context, Block},
    Frame,
};

const MIN_CANVAS_LENGTH: u16 = 3;

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
    // Reverse the string, because it is stored top->down, but is rendered bottom->up.
    for (y_offset, line) in text.lines().rev().enumerate() {
        context.print(f64::from(x), f64::from(y) + y_offset as f64, line, color);
    }
}

pub fn render(session: &mut Session, world: &mut World) -> Result<()> {
    session.terminal.draw(|frame| {
        let (actors_rect, ui_rect) = split_into_actors_and_ui(frame.size());
        let actors_viewport = create_actors_viewport(actors_rect);
        world.set_actors_viewport(actors_viewport);

        render_background(frame);
        render_canvas(
            frame,
            &mut world.actors,
            create_actors_block(),
            actors_rect,
            actors_viewport,
        );
        render_canvas(
            frame,
            &mut world.ui,
            create_ui_block(),
            ui_rect,
            create_ui_viewport(ui_rect),
        );
    })?;
    Ok(())
}

fn render_background<B: Backend>(frame: &mut Frame<B>) {
    // Set the background color of the *entire* terminal window, even outside of the canvas'.
    frame.render_widget(create_background_block(), frame.size());
}

fn render_canvas<B: Backend>(
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
    let mut canvas = create_canvas(block, x_min, x_max, y_min, y_max);
    let renderables = Rc::new(RefCell::new(renderables));
    canvas = canvas.paint(|ctx: &mut Context| {
        for renderable in renderables.borrow_mut().iter_mut() {
            renderable.render(ctx, viewport);
        }
    });
    frame.render_widget(canvas, rect);
}
