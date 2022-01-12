// Requires Rust from the nightly channel as of rustc 1.59.0-nightly (e012a191d 2022-01-06)
#![feature(mixed_integer_ops)]

use anyhow::Result;

use app::App;
use session::Session;

mod app;
mod clock;
mod color;
mod command;
mod game;
mod session;
mod view;

fn main() -> Result<()> {
    let mut session = Session::begin()?;

    App::default().run(&mut session)?;

    session.end()?;
    Ok(())
}
