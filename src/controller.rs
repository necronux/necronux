use crate::{
    bundle::loader::load_bundle_file,
    core::{
        commands::lvl_0::necronux::NecronuxCommand,
        error_reporter::init_error_reporter,
        handlers::init::init_handlers,
        logger::{init_logger, override_logger},
    },
};
use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};
use color_eyre::eyre::{Context, Result};
use log::{debug, error, info, trace, warn};

pub fn init_cli_controller() -> Result<()> {
    // Initializes the logger with default settings.
    let (mut logger_builder, logger, max_log_level) =
        init_logger().context("Failed to initialize the logger with default settings.")?;

    // Initializes the error reporter.
    init_error_reporter().context("Failed to initialize the error reporter.")?;

    // Initializes the cli parser.
    let cli = Cli::parse();
    debug!("Parsed CLI arguments: {:?}", cli);

    // Loads the necronux bundle file.
    load_bundle_file().context("Failed to load the necronux bundle file.")?;

    // Overrides the logger with merged log level setting from various sources.
    override_logger(&mut logger_builder, &logger, max_log_level, &cli)
        .context("Failed to override the logger with merged log level setting.")?;

    info!("Initializing handlers...");
    init_handlers(&cli)?;

    debug!("CLI controller initialization completed");

    // Flushes all logs from buffer before quitting.
    log::logger().flush();

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub necronux_command: Option<NecronuxCommand>,

    #[command(flatten)]
    pub verbose: Verbosity<WarnLevel>,
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
