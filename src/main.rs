mod commands;
mod controller;
mod handlers;
mod logger;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    controller::init_cli_controller()?;

    Ok(())
}
