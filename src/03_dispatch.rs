// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{Cli, HelpHandler, HelpType, SubCmd};
use anyhow::Result;

impl Cli {
    pub fn dispatch_subcmd(&self) -> Result<()> {
        match &self.shared.subcommand {
            Some(SubCmd::Bind(bind)) => {
                crate::BindHandler::new(bind, self.shared.progress).handle()
            }

            Some(SubCmd::Unbind(unbind)) => crate::UnbindHandler::new(unbind).handle(),

            Some(SubCmd::Validate(_)) => crate::ValidateHandler::new().handle(),

            Some(SubCmd::Status(status)) => crate::StatusHandler::new(status).handle(),

            None => HelpHandler::new(HelpType::RootHelp).handle(),
        }
    }
}
