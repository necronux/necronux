use color_eyre::{config::HookBuilder, eyre::Result};
use log::debug;

pub fn init_error_reporter() -> Result<()> {
    debug!("Initializing error reporter...");

    let backtrace = if cfg!(debug_assertions) { "full" } else { "0" };

    std::env::set_var("RUST_BACKTRACE", backtrace);
    HookBuilder::default()
        .display_env_section(false)
        .install()?;

    debug!("Error reporter initialized.");

    Ok(())
}
