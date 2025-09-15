// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{App, HelpHandler, HelpType, SubCmd, theme::ThemedUi};
use anyhow::Result;
use necronux::utils::trace_instrument;
use tracing::debug;

impl App {
    #[trace_instrument(level = "info", skip(self, theme))]
    pub fn handle_subcmd(&self, theme: &ThemedUi) -> Result<()> {
        debug!("Handling subcommands...");

        match &self.ctx.config.subcommand {
            Some(subcmd) => subcmd.handle(theme)?,
            None => HelpHandler::new(HelpType::RootHelp).handle(),
        }

        debug!("Handled subcommands successfully");
        Ok(())
    }
}

impl SubCmd {
    pub fn handle(&self, theme: &ThemedUi) -> Result<()> {
        match self {
            SubCmd::Bind(s) => s.handle(theme),
            SubCmd::Unbind(s) => s.handle(theme),
            SubCmd::Validate(s) => s.handle(theme),
            SubCmd::Status(s) => s.handle(theme),
        }
    }
}
