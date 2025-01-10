#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

mod commands;
mod controller;
mod error_reporter;
mod handlers;
mod logger;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    // Initializes the CLI controller.
    controller::init_cli_controller()?;

    Ok(())
}
