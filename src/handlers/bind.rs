// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    BindSubCmd, ValidateSubCmd,
    theme::{self, ThemedUi},
    utils,
};
use anyhow::{Context, Result, anyhow};
use necronux::{core::UnifiedGrimoire, pkg::StorageBackend, utils::trace_instrument};
use std::io::{stderr, stdout};
use tracing::{error, info};

impl BindSubCmd {
    #[trace_instrument(level = "info", name = "handle_bind", skip(self, theme), fields(source = %self.source, force = %self.force))]
    pub fn handle(&self, theme: &ThemedUi) -> Result<()> {
        info!("Handling bind request...");
        let t0 = std::time::Instant::now();

        let status = necronux::pkg::GrimoireBindingStatus::introspect()?;
        if status.is_bound && !self.force {
            error!("A grimoire is already bound; skipping bind");
            return Err(anyhow!(
                "A grimoire is already bound ({}@{}).\n\
                 Run `necronux unbind` to remove it, or use `--force` to override.",
                status.package_name.unwrap_or("<unknown>".into()),
                status.package_version.unwrap_or("<unknown>".into()),
            ));
        }

        if self.force {
            info!("Forcing grimoire binding by unbinding any bound grimoire...");
            crate::utils::remove_current_grimoire_dir()?;
        }

        info!("Binding grimoire from '{}'...", &self.source);
        let pb = task_msg!(
            theme,
            &mut stderr(),
            wants_spinner: true,
            has_steps: true,
            ("Binding grimoire from ", theme::style::progress_task),
            ("'", theme::style::regular),
            (self.source.to_string(), theme::style::underlined),
            ("'", theme::style::regular)
        )?;
        let result = Self::bind_grimoire(&self.source, theme)
            .with_context(|| format!("Failed to bind grimoire from '{}'", &self.source));
        if let Some(pb) = pb {
            pb.finish();
        }

        let grimoire = match result {
            Ok(g) => g,
            Err(e) => {
                error!("Cleaning current grimoire directory as binding failed...");
                crate::utils::remove_current_grimoire_dir()?;
                return Err(e);
            }
        };

        let name = grimoire
            .grimoire_metadata
            .as_ref()
            .and_then(|meta| meta.grimoire_name.as_deref())
            .map(|s| s.to_string())
            .unwrap_or(utils::missing_field_placeholder("name"));

        {
            let elapsed = t0.elapsed();
            let elapsed_fmt = crate::utils::format_duration(elapsed);
            info!(
                elapsed_secs = elapsed.as_secs_f64(),
                "Successfully bound grimoire '{}' from '{}'", name, &self.source
            );
            result_success_msg!(
                theme,
                &mut stdout(),
                Some(json_obj!(
                    "grimoire_name" => name.clone(),
                    "bind_elapsed" => elapsed_fmt.clone(),
                    "bind_elapsed_secs" => elapsed.as_secs_f64(),
                )),
                ("Grimoire bound ", theme::style::success),
                ("(", theme::style::regular),
                (elapsed_fmt.to_string(), theme::style::elapsed_time),
                (") - ", theme::style::regular),
                (name.to_string(), theme::style::bold),
                (" is ready for use", theme::style::dim),
            )?;
            Ok(())
        }
    }

    #[trace_instrument(level = "info", skip(theme), fields(source = %source))]
    fn bind_grimoire(source: &str, theme: &ThemedUi) -> Result<UnifiedGrimoire> {
        step_msg!(
            theme,
            &mut stderr(),
            wants_spinner: false,
            ("Resolving storage backend...", theme::style::progress_step),
        )?;
        let storage_backend = necronux::pkg::resolve_storage_backend(source)?;

        let pb = step_msg!(
            theme,
            &mut stderr(),
            wants_spinner: true,
            ("Fetching grimoire...", theme::style::progress_step),
        )?;
        let fetched = storage_backend.fetch(source)?;
        if let Some(pb) = pb {
            pb.finish();
        }

        step_msg!(
            theme,
            &mut stderr(),
            wants_spinner: false,
            ("Extracting grimoire...", theme::style::progress_step),
        )?;
        let current_grimoire_path = necronux::utils::paths::current_grimoire_path()?;
        let extraction_path = current_grimoire_path.join(&fetched.package_name);
        necronux::utils::zip::extract_zip(
            &fetched.fetched_package_zip_path,
            "fetched grimoire package",
            &extraction_path,
        )?;

        let pb = step_msg!(
            theme,
            &mut stderr(),
            wants_spinner: true,
            ("Validating grimoire...", theme::style::progress_step),
        )?;
        let grimoire =
            ValidateSubCmd::validate_grimoire().context("Failed to validate grimoire")?;
        if let Some(pb) = pb {
            pb.finish();
        }

        Ok(grimoire)
    }
}
