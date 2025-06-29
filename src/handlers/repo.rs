// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{Connect, Switch};
use anyhow::Context;

pub struct RepoHandler;

impl RepoHandler {
    pub fn handle_connect(connect: &Connect) -> anyhow::Result<()> {
        Connect::handle_connect(connect)
    }
    pub fn handle_switch(switch: &Switch) -> anyhow::Result<()> {
        Switch::handle_switch(switch)
    }

    pub fn handle_reconnect() -> anyhow::Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::info_span!("handle_reconnect").entered();

        let url = necronux_core::vcs::status::get_remote_url()?;
        necronux_core::vcs::fs::clone_repo(&url)
    }

    pub fn handle_disconnect() -> anyhow::Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::info_span!("handle_disconnect").entered();

        necronux_core::vcs::fs::delete_active_clone()
    }

    pub fn handle_update() -> anyhow::Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::info_span!("handle_update").entered();

        necronux_core::vcs::fs::update_repo()
    }

    pub fn handle_status() -> anyhow::Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::info_span!("handle_status").entered();

        necronux_core::vcs::status::display_repo_status()
    }
}

impl Connect {
    fn handle_connect(&self) -> anyhow::Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::info_span!("handle_connect").entered();

        if necronux_core::vcs::status::is_repo_connected()
            .context("Failed to determine repo connection status")?
        {
            println!("A repository is already connected.");
            return Ok(());
        }

        necronux_core::vcs::fs::clone_repo(&self.url)?;

        if let Some(revision) = &self.revision {
            match necronux_core::vcs::fs::checkout_revision(revision) {
                Ok(_) => {
                    println!("Successfully connected to repository at revision '{revision}'");
                }
                Err(e) => {
                    let _ = necronux_core::vcs::fs::delete_active_clone();
                    return Err(anyhow::anyhow!("{e:#}"));
                }
            }
        } else {
            println!("Successfully connected to repository");
        }
        Ok(())
    }
}

impl Switch {
    fn handle_switch(&self) -> anyhow::Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::info_span!("handle_switch").entered();

        necronux_core::vcs::fs::clone_repo(&self.url)?;

        if let Some(revision) = &self.revision {
            match necronux_core::vcs::fs::checkout_revision(revision) {
                Ok(_) => {
                    println!("Successfully connected to repository at revision '{revision}'");
                }
                Err(e) => {
                    let _ = necronux_core::vcs::fs::delete_active_clone();
                    return Err(anyhow::anyhow!("{e:#}"));
                }
            }
        } else {
            println!("Successfully connected to repository");
        }
        Ok(())
    }
}
