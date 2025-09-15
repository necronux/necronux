// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

#[cfg(feature = "necronux_log")]
use crate::LogContext;
use crate::{App, UiContext, theme::ThemedUi, utils};
use anyhow::Result;
use tracing::{debug, info};

impl App {
    pub fn run(&self, t0: std::time::Instant) -> Result<()> {
        #[cfg(feature = "necronux_log")]
        LogContext::new(&self.ctx).init_if_enabled()?;

        // Start logging and tracing from this point

        debug!(
            "Parsed CLI: {:?} (parsing occurred before logging was initialized)",
            self.ctx.cli_opts
        );
        utils::log_sys_info(&self.ctx.sys);

        let ui_context = UiContext::new(&self.ctx);
        ui_context.init();

        let themed_ui = ThemedUi(ui_context);

        #[cfg(feature = "experimental_pretty_cli")]
        if self.ctx.config.help {
            crate::HelpHandler::new(crate::HelpType::RootHelp).handle()
        }

        self.handle_subcmd(&themed_ui)?;

        // Closing block
        {
            let elapsed = t0.elapsed();
            debug!(
                elapsed_time_ms = %elapsed.as_millis(),
                "CLI run took {}ms", elapsed.as_millis()
            );
            let elapsed_fmt = utils::format_duration(elapsed);
            info!(
                elapsed_time = %elapsed_fmt,
                "CLI run completed successfully"
            );

            #[cfg(all(feature = "necronux_log", debug_assertions))]
            necronux::log::maybe_flush();

            Ok(())
        }
    }
}
