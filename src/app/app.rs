use super::command::Command;
use crate::{
    clock::ticker::{TickHandler, Ticker},
    game::world::World,
    view::{
        factory::{
            create_actors_block, create_actors_viewport, create_background_block, create_ui_block,
            create_ui_viewport, split_into_actors_and_ui,
        },
        render::render_canvas,
        session::Session,
    },
};
use anyhow::Result;
use crossterm::event::{poll, read, Event};
use std::time::Duration;

const TICK_RATE_MS: u64 = 20;

pub struct App {
    ticker: Ticker,
    world: World,
}

impl Default for App {
    fn default() -> Self {
        Self {
            ticker: Ticker::new(Duration::from_millis(TICK_RATE_MS)),
            world: World::default(),
        }
    }
}
impl App {
    pub fn run(&mut self, session: &mut Session) -> Result<()> {
        loop {
            if self.ticker.maybe_tick() {
                self.world.handle_tick(&self.ticker);
            }
            self.render(session)?;

            let mut commands = self.world.detect_collisions();

            match self.wait_for_input_command()? {
                Some(command) => match command {
                    Command::Quit => return Ok(()),
                    Command::Restart => self.ticker.restart(),
                    _ => commands.push(command),
                },
                None => (),
            }
            self.world.broadcast_commands(commands)?;
        }
    }

    fn render(&mut self, session: &mut Session) -> Result<()> {
        session.terminal.draw(|frame| {
            let window = frame.size();
            // Set the background color of the entire terminal window.
            frame.render_widget(create_background_block(), window);

            let (actors_rect, ui_rect) = split_into_actors_and_ui(window);
            let actors_viewport = create_actors_viewport(actors_rect);
            let ui_viewport = create_ui_viewport(actors_rect);
            self.world.set_actors_viewport(actors_viewport);
            render_canvas(
                frame,
                &mut self.world.actors,
                create_actors_block(),
                actors_rect,
                actors_viewport,
            );
            render_canvas(
                frame,
                &mut self.world.ui,
                create_ui_block(),
                ui_rect,
                ui_viewport,
            );
        })?;
        Ok(())
    }

    fn wait_for_input_command(&mut self) -> Result<Option<Command>> {
        if poll(self.ticker.remaining_timeout())? {
            // `poll()` returned true, so an event is available,
            // so the following call to `read()` will not block.
            if let Event::Key(key) = read()? {
                return Ok(Some(Command::from(key)));
            }
        }
        Ok(None)
    }
}
