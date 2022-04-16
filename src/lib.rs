use anyhow::Result;
use app::main::App;
use view::session::Session;

mod app;
mod clock;
mod game;
mod view;

pub fn run() -> Result<()> {
    let mut session = Session::begin()?;

    App::default().run(&mut session)?;

    Ok(session.end()?)
}
