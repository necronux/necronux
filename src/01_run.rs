// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{Cli, ui::UiSetup};
use anyhow::Result;
use tracing::debug;

impl Cli {
    pub fn run() -> Result<()> {
        let cli = Cli::init();

        #[cfg(feature = "necronux_log")]
        {
            use anyhow::Context;

            #[cfg(feature = "cli_arg_log")]
            necronux_core::log::LogSetup::from_cli_or_with_default_level(
                cli.verbosity,
                cli.shared.no_color,
                #[cfg(debug_assertions)]
                cli.shared.log_extended,
            )
            .init()
            .context("Failed to setup logging")?;
            #[cfg(not(feature = "cli_arg_log"))]
            necronux_core::log::LogSetup::with_default_level(
                cli.shared.no_color,
                #[cfg(debug_assertions)]
                cli.shared.log_extended,
            )
            .init()
            .context("Failed to setup logging")?;
        }

        // Start logging and tracing from this point

        debug!("Parsed CLI: {cli:?} (parsing occurred before logging was initialized)");

        UiSetup::new(cli.shared.no_color).init()?;

        #[cfg(feature = "experimental_pretty_cli")]
        if cli.help {
            crate::HelpHandler::new(crate::HelpType::RootHelp).handle()
        }

        Cli::dispatch_subcmd(&cli)
    }
}
