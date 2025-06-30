// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{Cli, HelpHandler};

impl Cli {
    pub fn dispatch_subcmd(&self) -> anyhow::Result<()> {
        match &self.subcommand {
            Some(crate::SubCmd::Connect(_)) => {
                use anyhow::Context;
                crate::ConnectHandler::handle_connect().context("Failed to connect")
            }

            Some(crate::SubCmd::Validate(_)) => {
                use anyhow::Context;
                crate::ValidateHandler::handle_validate().context("Failed to validate")
            }

            None => HelpHandler::new(crate::HelpType::RootHelp).handle(),
        }
    }
}
