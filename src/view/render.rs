use super::{
    coordinates::Coordinates,
    factory::{
        create_canvas, create_resize_warning_paragraph, WINDOW_MIN_HEIGHT, WINDOW_MIN_WIDTH,
    },
    renderer::Renderer,
    viewport::Viewport,
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
use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{canvas::Context, Block},
    Frame,
};

const MIN_CANVAS_LENGTH: u16 = 3;

pub trait Renderable {
    fn render(&self, renderer: &mut Renderer);

    fn viewport(&self) -> Viewport;
}

pub fn render(session: &mut Session, world: &mut World) -> Result<()> {
    session.terminal.draw(|frame| {
        let window = frame.size();
        let (actors_rect, ui_rect) = split_into_actors_and_ui(window);

        if should_resize_window(window) {
            frame.render_widget(create_resize_warning_paragraph(), window);
            return;
        }

        let actors_viewport = create_actors_viewport(actors_rect);
        let ui_viewport = create_ui_viewport(ui_rect);
        world
            .broadcast_actors_viewport(actors_viewport)
            .expect("Broadcast Actors Viewport succeeds");
        world
            .broadcast_ui_viewport(ui_viewport)
            .expect("Broadcast UI Viewport succeeds");

        render_background(frame);
        render_canvas(
            frame,
            &world.actors,
            create_actors_block(),
            world.offset,
            actors_rect,
            actors_viewport,
        );
        render_canvas(
            frame,
            &world.ui,
            create_ui_block(),
            Coordinates::default(),
            ui_rect,
            ui_viewport,
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
    renderables: &[Box<dyn GameItem>],
    block: Block,
    offset: Coordinates,
    rect: Rect,
    viewport: Viewport,
) {
    if is_canvas_too_small_to_render(rect) {
        return;
    }
    let mut canvas = create_canvas(block, viewport);
    canvas = canvas.paint(|ctx: &mut Context| {
        let mut renderer = Renderer::new(ctx, offset, viewport);
        for renderable in renderables.iter() {
            renderable.render(&mut renderer);
        }
    });
    frame.render_widget(canvas, rect);
}

fn is_canvas_too_small_to_render(canvas: Rect) -> bool {
    canvas.width < MIN_CANVAS_LENGTH || canvas.height < MIN_CANVAS_LENGTH
}

fn should_resize_window(window: Rect) -> bool {
    window.height < u16::from(WINDOW_MIN_HEIGHT) || window.width < u16::from(WINDOW_MIN_WIDTH)
}
