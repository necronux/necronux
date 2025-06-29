// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::Cli;
use tracing::debug;

impl Cli {
    pub fn run() -> anyhow::Result<()> {
        let cli = Cli::init();

        #[cfg(feature = "necronux_log")]
        {
            use anyhow::Context;

            #[cfg(feature = "cli_arg_log")]
            necronux_core::log::LogSetup::from_cli_or_default(cli.verbosity)
                .init()
                .context("Failed to setup logging")?;
            #[cfg(not(feature = "cli_arg_log"))]
            necronux_core::log::LogSetup::default()
                .init()
                .context("Failed to setup logging")?;
        }

        // Start logging and tracing from this point

        debug!("Parsed CLI: {cli:?} (parsing occurred before logging was initialized)");

        #[cfg(feature = "experimental_pretty_cli")]
        if cli.help {
            crate::HelpHandler::new(crate::HelpType::RootHelp).handle()
        }

        Cli::dispatch_subcmd(&cli)
    }
}
