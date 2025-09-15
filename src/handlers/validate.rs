// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    ValidateSubCmd,
    theme::{self, ThemedUi},
    utils,
};
use anyhow::{Context, Result, anyhow};
use necronux::{
    core::{GrimoireValidator, TryIntoUnifiedGrimoire, UnifiedGrimoire},
    utils::trace_instrument,
};
use std::io::{stderr, stdout};
use tracing::{error, info};

impl ValidateSubCmd {
    #[trace_instrument(level = "info", name = "handle_validate", skip(self, theme))]
    pub fn handle(&self, theme: &ThemedUi) -> Result<()> {
        info!("Handling validate request...");
        let t0 = std::time::Instant::now();

        let status = necronux::pkg::GrimoireBindingStatus::introspect()?;
        if !status.is_bound {
            error!("No grimoire is currently bound; skipping validate");
            return Err(anyhow!(
                "No grimoire is currently bound. Nothing to validate.\n\
                Use `necronux bind <path-or-url>` to bind a grimoire first."
            ));
        }

        let pb = task_msg!(
            theme,
            &mut stderr(),
            wants_spinner: true,
            has_steps: false,
            ("Validating grimoire...", theme::style::progress_task)
        )?;
        let grimoire = Self::validate_grimoire().context("Failed to validate grimoire")?;
        if let Some(pb) = pb {
            pb.finish();
        }

        let name = grimoire
            .grimoire_metadata
            .as_ref()
            .and_then(|meta| meta.grimoire_name.as_deref())
            .map(|s| s.to_string())
            .unwrap_or_else(|| utils::missing_field_placeholder("name"));

        {
            let elapsed = t0.elapsed();
            let elapsed_fmt = crate::utils::format_duration(elapsed);
            info!(
                elapsed_secs = elapsed.as_secs_f64(),
                "Successfully validated grimoire '{}'", name
            );
            result_success_msg!(
                theme,
                &mut stdout(),
                Some(json_obj!(
                    "grimoire_name" => name.clone(),
                    "validate_elapsed" => elapsed_fmt.clone(),
                    "validate_elapsed_secs" => elapsed.as_secs_f64(),
                )),
                (name.to_string(), theme::style::success),
                (" is valid ", theme::style::success),
                ("(", theme::style::regular),
                (elapsed_fmt.to_string(), theme::style::elapsed_time),
                (")", theme::style::regular),
            )?;
            Ok(())
        }
    }

    #[trace_instrument(level = "info")]
    pub fn validate_grimoire() -> Result<UnifiedGrimoire> {
        let parsed = necronux::core::resolve_grimoire_parser()?;
        parsed.validate()?;
        Ok(parsed.into_unified())
    }
}
