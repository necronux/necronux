// ==----------------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2024-2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==----------------------------------------------------------------== //

use crate::{commands::lvl_0::necronux::NecronuxCommand, handlers::init::init_handlers};
use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};
use color_eyre::eyre::{Context, Result};
use log::{debug, info};

pub fn init_flow_controller() -> Result<()> {
    // Initializes the logger with default settings.
    let (mut logger_builder, logger, max_log_level) = necronux_utils::logger::init_logger(None)
        .context("Failed to initialize the logger with default settings.")?;
    debug!(
        "For cli verbosity flags WARN is the reference log level. \
        '-v' increases verbosity from this level, and '-q' decreases it."
    );

    // Initializes the error reporter.
    necronux_utils::error_reporter::init_error_reporter()
        .context("Failed to initialize the error reporter.")?;

    // Initializes the cli parser.
    let cli = Cli::parse();
    debug!("Parsed CLI arguments: {:?}", cli);

    // Loads the necronux bundle file.
    necronux_core::bundle::loader::load_bundle_file()
        .context("Failed to load the necronux bundle file.")?;

    // Overrides the logger with merged log level setting from various sources.
    crate::logger::override_logger(&mut logger_builder, &logger, max_log_level, &cli)
        .context("Failed to override the logger with merged log level setting.")?;

    info!("Initializing handlers...");
    init_handlers(&cli)?;

    debug!("CLI controller initialization completed");

    // Flushes all logs from buffer before quitting.
    log::logger().flush();

    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "Necronux", version, about, long_about = None)]
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
