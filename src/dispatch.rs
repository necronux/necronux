// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{Cli, HelpHandler};

impl Cli {
    pub fn dispatch_subcmd(&self) -> anyhow::Result<()> {
        match &self.subcommand {
            #[cfg(feature = "experimental_engine")]
            Some(crate::SubCmd::Init(_)) => HelpHandler::new(crate::HelpType::RootHelp).handle(),

            #[cfg(feature = "experimental_auth")]
            Some(crate::SubCmd::Auth(_)) => HelpHandler::new(crate::HelpType::RootHelp).handle(),

            Some(crate::SubCmd::Repo(repo_subcmd)) => match &repo_subcmd.subcommand {
                Some(crate::RepoSubSubCmd::Connect(connect)) => {
                    use anyhow::Context;
                    crate::RepoHandler::handle_connect(connect).context("Failed to connect repo")
                }
                Some(crate::RepoSubSubCmd::Disconnect) => {
                    use anyhow::Context;
                    crate::RepoHandler::handle_disconnect().context("Failed to disconnect repo")
                }
                Some(crate::RepoSubSubCmd::Reconnect) => {
                    use anyhow::Context;
                    crate::RepoHandler::handle_reconnect().context("Failed to reconnect repo")
                }
                Some(crate::RepoSubSubCmd::Switch(switch)) => {
                    use anyhow::Context;
                    crate::RepoHandler::handle_switch(switch).context("Failed to switch repo")
                }
                Some(crate::RepoSubSubCmd::Update) => {
                    use anyhow::Context;
                    crate::RepoHandler::handle_update().context("Failed to update repo")
                }
                Some(crate::RepoSubSubCmd::Status) => {
                    use anyhow::Context;
                    crate::RepoHandler::handle_status().context("Failed to display repo status")
                }
                None => HelpHandler::new(crate::HelpType::RepoHelp).handle(),
            },

            #[cfg(feature = "stdschema_v1")]
            Some(crate::SubCmd::Validate(_)) => {
                use anyhow::Context;
                crate::ValidateHandler::handle_validate().context("Failed to validate")
            }

            #[cfg(feature = "experimental_engine")]
            Some(crate::SubCmd::Apply(_)) => HelpHandler::new(crate::HelpType::RootHelp).handle(),

            #[cfg(feature = "experimental_engine")]
            Some(crate::SubCmd::Revert(_)) => HelpHandler::new(crate::HelpType::RootHelp).handle(),

            #[cfg(feature = "experimental_engine")]
            Some(crate::SubCmd::Verify(_)) => HelpHandler::new(crate::HelpType::RootHelp).handle(),

            #[cfg(feature = "experimental_discovery")]
            Some(crate::SubCmd::Search(_)) => HelpHandler::new(crate::HelpType::RootHelp).handle(),

            #[cfg(feature = "experimental_discovery")]
            Some(crate::SubCmd::Catalog(_)) => HelpHandler::new(crate::HelpType::RootHelp).handle(),

            #[cfg(feature = "experimental_discovery")]
            Some(crate::SubCmd::Explore(_)) => HelpHandler::new(crate::HelpType::RootHelp).handle(),

            #[cfg(feature = "experimental_discovery")]
            Some(crate::SubCmd::Info(_)) => HelpHandler::new(crate::HelpType::RootHelp).handle(),

            #[cfg(feature = "experimental_engine")]
            Some(crate::SubCmd::Status(_)) => HelpHandler::new(crate::HelpType::RootHelp).handle(),

            #[cfg(feature = "experimental_history")]
            Some(crate::SubCmd::History(_)) => HelpHandler::new(crate::HelpType::RootHelp).handle(),

            #[cfg(feature = "experimental_engine")]
            Some(crate::SubCmd::Clean(_)) => HelpHandler::new(crate::HelpType::RootHelp).handle(),

            None => HelpHandler::new(crate::HelpType::RootHelp).handle(),
        }
    }
}
