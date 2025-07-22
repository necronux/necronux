// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{UnbindSubCmd, ui};
use anyhow::{Context, Result, anyhow};

pub struct UnbindHandler {
    force: bool,
}

impl UnbindHandler {
    pub fn new(subcmd: &UnbindSubCmd) -> Self {
        Self {
            force: subcmd.force,
        }
    }

    pub fn handle(self) -> Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("handle_unbind").entered();

        let status = necronux_core::pkg::GrimoireBindingStatus::introspect()?;
        if !status.is_bound && !self.force {
            return Err(anyhow!(
                "No grimoire is currently bound. Nothing to unbind.\nUse `necronux bind <path-or-url>` to bind a grimoire first."
            ));
        }

        let pb = ui::msg::task_spinner_or_msg(
            false,
            false,
            ui::style::progress_task("Unbinding grimoire...").to_string(),
        )?;

        let t0 = std::time::Instant::now();

        Self::unbind_grimoire().context("Failed to unbind grimoire")?;

        if let Some(pb) = pb {
            pb.finish();
        }

        let elapsed = t0.elapsed();
        let elapsed_fmt = ui::format_duration(elapsed);
        println!(
            "{} {} ({})",
            ui::symbol::success(),
            ui::style::success("Grimoire unbound"),
            ui::style::elapsed_time(&elapsed_fmt),
        );
        Ok(())
    }

    fn unbind_grimoire() -> Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("unbind_grimoire").entered();

        let current_grimoire_path = necronux_core::utils::paths::current_grimoire_path()?;
        necronux_core::utils::fs::remove_dir_all_if_exists(
            &current_grimoire_path,
            "current grimoire",
        )?;
        Ok(())
    }
}
