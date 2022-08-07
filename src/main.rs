#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::trait_duplication_in_bounds)]

use anyhow::Result;

mod cli;
mod junit_merger;

fn main() -> Result<()> {
    cli::Application::new().run()
}
