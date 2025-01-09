mod commands;
mod controller;
mod error_reporter;
mod handlers;
mod logger;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    // Intializes the CLI controller.
    controller::init_cli_controller()?;

    Ok(())
}
