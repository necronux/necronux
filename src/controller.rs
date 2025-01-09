use crate::commands::lvl_0::necronux::NecronuxCommand;
use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};
use color_eyre::eyre::Result;
use log::{debug, info};

pub fn init_cli_controller() -> Result<()> {
    // Intializes the logger.
    crate::logger::init_logger()?;

    // Initializes error reporter.
    crate::error_reporter::init_error_reporter()?;

    // Initializes cli parser.
    let cli = Cli::parse();
    debug!("Parsed CLI arguments: {:?}", cli);

    // Overrides the logger.
    crate::logger::override_logger()?;

    info!("Initializing handlers");
    crate::handlers::init::init_handlers(&cli)?;

    debug!("Cli controller initialization completed");

    // Flush all logs from buffer before quitting.
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
