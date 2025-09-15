// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    UnbindSubCmd,
    theme::{self, ThemedUi},
};
use anyhow::{Context, Result, anyhow};
use necronux::utils::trace_instrument;
use std::io::{stderr, stdout};
use tracing::{error, info};

impl UnbindSubCmd {
    #[trace_instrument(level = "info", name = "handle_unbind", skip(self, theme), fields(force = %self.force))]
    pub fn handle(&self, theme: &ThemedUi) -> Result<()> {
        info!("Handling unbind request...");
        let t0 = std::time::Instant::now();

        let status = necronux::pkg::GrimoireBindingStatus::introspect()?;
        if !status.is_bound && !self.force {
            error!("No grimoire is currently bound; skipping unbind");
            return Err(anyhow!(
                "No grimoire is currently bound. Nothing to unbind.\n\
                Use `necronux bind <path-or-url>` to bind a grimoire first."
            ));
        }

        let pb = task_msg!(
            theme,
            &mut stderr(),
            wants_spinner: true,
            has_steps: false,
            ("Unbinding grimoire...", theme::style::progress_task)
        )?;
        Self::unbind_grimoire().context("Failed to unbind grimoire")?;
        if let Some(pb) = pb {
            pb.finish();
        }

        {
            let elapsed = t0.elapsed();
            let elapsed_fmt = crate::utils::format_duration(elapsed);
            info!(
                elapsed_secs = elapsed.as_secs_f64(),
                "Successfully unbound grimoire"
            );
            result_success_msg!(
                theme,
                &mut stdout(),
                Some(json_obj!(
                    "unbind_elapsed" => elapsed_fmt.clone(),
                    "unbind_elapsed_secs" => elapsed.as_secs_f64(),
                )),
                ("Grimoire unbound", theme::style::success),
                (" (", theme::style::regular),
                (elapsed_fmt.to_string(), theme::style::elapsed_time),
                (")", theme::style::regular),
            )?;
            Ok(())
        }
    }

    #[trace_instrument(level = "info")]
    fn unbind_grimoire() -> Result<()> {
        crate::utils::remove_current_grimoire_dir()
    }
}
