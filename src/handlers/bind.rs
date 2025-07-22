// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::ValidateHandler;
use crate::{BindSubCmd, ui};
use anyhow::{Context, Result, anyhow};
use necronux_core::{engine::UnifiedGrimoire, pkg::StorageBackend};
use tracing::info;

pub struct BindHandler {
    source: String,
    force: bool,
    progress: bool,
}

impl BindHandler {
    pub fn new(subcmd: &BindSubCmd, progress: bool) -> Self {
        Self {
            source: subcmd.source.clone(),
            force: subcmd.force,
            progress,
        }
    }

    pub fn handle(self) -> Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("handle_bind").entered();

        let status = necronux_core::pkg::GrimoireBindingStatus::introspect()?;
        if status.is_bound && !self.force {
            return Err(anyhow!(
                "A grimoire is already bound ({}@{}).\n\
                 Run `necronux unbind` to remove it, or use `--force` to override.",
                status.package_name.unwrap_or("<unknown>".into()),
                status.package_version.unwrap_or("<unknown>".into()),
            ));
        }

        if self.force {
            let current_grimoire_path = necronux_core::utils::paths::current_grimoire_path()?;
            necronux_core::utils::fs::remove_dir_all_if_exists(
                &current_grimoire_path,
                "current grimoire",
            )?;
        }

        let pb = ui::msg::task_spinner_or_msg(
            true,
            self.progress,
            format!(
                "{} '{}'",
                ui::style::progress_task("Binding grimoire from"),
                console::style(&self.source).underlined()
            ),
        )?;

        let t0 = std::time::Instant::now();

        let result = Self::bind_grimoire(&self.source, self.progress)
            .with_context(|| format!("Failed to bind grimoire from '{}'", &self.source));

        let grimoire = match result {
            Ok(g) => g,
            Err(e) => {
                info!("Cleaning current grimoire directory as binding failed...");

                let current_grimoire_path = necronux_core::utils::paths::current_grimoire_path()?;
                necronux_core::utils::fs::remove_dir_all_if_exists(
                    &current_grimoire_path,
                    "current grimoire",
                )?;
                return Err(e);
            }
        };

        if let Some(pb) = pb {
            pb.finish();
        }

        let fallback = ui::no("name");
        let name = grimoire
            .grimoire_metadata
            .as_ref()
            .and_then(|meta| meta.grimoire_name.as_deref())
            .unwrap_or(&fallback);

        let elapsed = t0.elapsed();
        let elapsed_fmt = ui::format_duration(elapsed);

        println!(
            "{} {} ({}) — {} {}",
            ui::symbol::success(),
            ui::style::success("Grimoire bound"),
            ui::style::elapsed_time(&elapsed_fmt),
            console::style(name).bold(),
            console::style("is ready for use").dim(),
        );

        Ok(())
    }

    fn bind_grimoire(source: &str, progress: bool) -> Result<UnifiedGrimoire> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("bind_grimoire", source = source).entered();

        ui::msg::step_msg(
            progress,
            ui::style::progress_step("Resolving storage backend...").to_string(),
        )?;
        let storage_backend = necronux_core::pkg::resolve_storage_backend(source)?;

        let pb = ui::msg::step_spinner(
            progress,
            ui::style::progress_step("Fetching grimoire...").to_string(),
        )?;
        let fetched = storage_backend.fetch(source)?;
        if let Some(pb) = pb {
            pb.finish();
        }

        ui::msg::step_msg(
            progress,
            ui::style::progress_step("Extracting grimoire...").to_string(),
        )?;
        let current_grimoire_path = necronux_core::utils::paths::current_grimoire_path()?;
        let extraction_path = current_grimoire_path.join(&fetched.package_name);
        necronux_core::utils::zip::extract_zip_if_exists(
            &fetched.fetched_package_zip_path,
            "fetched grimoire package",
            &extraction_path,
        )?;

        let pb = ui::msg::step_spinner(
            progress,
            ui::style::progress_step("Validating grimoire...").to_string(),
        )?;
        let grimoire =
            ValidateHandler::validate_grimoire().context("Failed to validate grimoire")?;
        if let Some(pb) = pb {
            pb.finish();
        }

        Ok(grimoire)
    }
}
