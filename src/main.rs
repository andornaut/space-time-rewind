// Requires Rust from the nightly channel as of rustc 1.59.0-nightly (e012a191d 2022-01-06)
//#![feature(mixed_integer_ops)]
// Allow modules to have the same name as their parent module
//#![allow(clippy::module_inception)]
// Show more lint warnings
#![warn(clippy::all, clippy::pedantic)]
use anyhow::Result;
use space_time_rewind::run;

fn main() -> Result<()> {
    run()
}
