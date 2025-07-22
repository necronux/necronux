// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::ui;
use anyhow::{Context, Result, anyhow};
use necronux_core::engine::{GrimoireValidator, TryIntoUnifiedGrimoire, UnifiedGrimoire};

pub struct ValidateHandler {}

impl Default for ValidateHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidateHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle(self) -> Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("handle_validate").entered();

        let status = necronux_core::pkg::GrimoireBindingStatus::introspect()?;
        if !status.is_bound {
            return Err(anyhow!(
                "No grimoire is currently bound. Nothing to validate.\nUse `necronux bind <path-or-url>` to bind a grimoire first."
            ));
        }

        let pb = ui::msg::task_spinner_or_msg(
            false,
            false,
            ui::style::progress_task("Validating grimoire...").to_string(),
        )?;

        let t0 = std::time::Instant::now();

        let grimoire = Self::validate_grimoire().context("Failed to validate grimoire")?;

        if let Some(pb) = pb {
            pb.finish();
        }

        let elapsed = t0.elapsed();
        let elapsed_fmt = ui::format_duration(elapsed);

        let fallback = ui::no("name");
        let name = grimoire
            .grimoire_metadata
            .as_ref()
            .and_then(|meta| meta.grimoire_name.as_deref())
            .unwrap_or(&fallback);

        println!(
            "{} {} {} ({})",
            ui::symbol::success(),
            ui::style::success(name),
            ui::style::success("is valid"),
            ui::style::elapsed_time(&elapsed_fmt),
        );
        Ok(())
    }

    pub fn validate_grimoire() -> Result<UnifiedGrimoire> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("validate_grimoire").entered();

        let parsed = necronux_core::engine::resolve_grimoire_parser()?;
        let _ = &parsed.validate()?;
        let unified = parsed.try_into_unified()?;
        Ok(unified)
    }
}
