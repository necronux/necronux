// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    VerifySubCmd,
    theme::{self, ThemedUi},
};
use anyhow::{Context, Result};
use necronux::utils::trace_instrument;
use std::io::{stderr, stdout};
use tracing::info;

impl VerifySubCmd {
    #[trace_instrument(level = "info", name = "handle_verify", skip(self, theme))]
    pub fn handle(&self, theme: &ThemedUi) -> Result<()> {
        info!("Handling verify request...");
        let t0 = std::time::Instant::now();

        let pb = task_msg!(
            theme,
            &mut stderr(),
            wants_spinner: true,
            has_steps: false,
            ("Verifying...", theme::style::progress_task)
        )?;
        Self::verify().context("Failed to verify")?;
        if let Some(pb) = pb {
            pb.finish();
        }

        {
            let elapsed = t0.elapsed();
            let elapsed_fmt = crate::utils::format_duration(elapsed);
            info!(elapsed_secs = elapsed.as_secs_f64(), "Successfully verify");
            result_success_msg!(
                theme,
                &mut stdout(),
                Some(json_obj!(
                    "verify_elapsed" => elapsed_fmt.clone(),
                    "verify_elapsed_secs" => elapsed.as_secs_f64(),
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
    fn verify() -> Result<()> {
        Ok(())
    }
}
