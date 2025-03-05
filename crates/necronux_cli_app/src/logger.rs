// ==----------------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2024-2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==----------------------------------------------------------------== //

use crate::flow_controller::Cli;
use color_eyre::eyre::Result;
use flexi_logger::{LogSpecBuilder, LoggerHandle};
use log::{LevelFilter, debug};
use std::env;

pub fn override_logger(
    logger_builder: &mut LogSpecBuilder,
    logger: &LoggerHandle,
    max_log_level: LevelFilter,
    cli: &Cli,
) -> Result<()> {
    // The order of precedence is:
    // Default < Necronux Bundle File < Environment Variable < CLI Verbosity Flag
    debug!("Loading log level setting from various sources...");

    let mut log_level_updated = false;

    // Override log level from the necronux bundle file.
    //
    //  TODO
    //

    // Override log level from the environment variable.
    if let Ok(env_log_level_str) = env::var("NECRONUX_LOG_LEVEL") {
        match env_log_level_str.as_str() {
            "TRACE" => Some(LevelFilter::Trace),
            "DEBUG" => Some(LevelFilter::Debug),
            "INFO" => Some(LevelFilter::Info),
            "WARN" => Some(LevelFilter::Warn),
            "ERROR" => Some(LevelFilter::Error),
            "OFF" => Some(LevelFilter::Off),
            _ => None,
        }
        .map_or_else(
            || {
                debug!(
                    "Invalid log level {} in NECRONUX_LOG_LEVEL. Ignoring...",
                    env_log_level_str
                );
            },
            |env_log_level| {
                debug!(
                    "Overriding log level to {} from environment variable.",
                    env_log_level
                );
                logger_builder.default(env_log_level);
                log_level_updated = true;
            },
        );
    }

    // Override log level from the cli verbosity flag.
    if cli.verbose.is_present() {
        let cli_log_level = cli.verbose.log_level_filter();
        debug!(
            "Overriding log level to {} from cli verbosity flag.",
            cli_log_level
        );
        logger_builder.default(cli_log_level);
        log_level_updated = true;
    }

    // If there were any updates, apply the new specification to the logger.
    if log_level_updated {
        let new_log_level = logger_builder.build();
        logger.set_new_spec(new_log_level.clone());

        let new_log_level_str = new_log_level.to_string().to_uppercase();
        debug!("Log level updated to {}.", new_log_level_str);

        // Set max log level again.
        log::set_max_level(max_log_level);
        let overridden_max_log_level = log::max_level();
        debug!(
            "Max log level set to {} for the overridden log config.",
            overridden_max_log_level
        );

        debug!("Log level setting loaded from various sources and merged.");
    } else {
        debug!("No log level updates applied, no overrides found in various sources.");
    }

    Ok(())
}
