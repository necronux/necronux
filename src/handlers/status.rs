// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    StatusSubCmd, ValidateSubCmd,
    theme::{self, ThemedUi},
    utils,
};
use anyhow::{Context, Result};
use necronux::utils::trace_instrument;
use std::io::{Write, stderr, stdout};
use tracing::info;

impl StatusSubCmd {
    #[trace_instrument(level = "info", name = "handle_status", skip(self, theme), fields(full = %self.full))]
    pub fn handle(&self, theme: &ThemedUi) -> Result<()> {
        info!("Handling status request...");
        let t0 = std::time::Instant::now();

        info!("Showing status...");
        Self::show_status(self.full, theme).context("Failed to show status")?;

        {
            let elapsed = t0.elapsed();
            let elapsed_fmt = crate::utils::format_duration(elapsed);
            info!(
                elapsed_secs = elapsed.as_secs_f64(),
                "Successfully shown status"
            );
            result_success_msg!(
                theme,
                &mut stdout(),
                Some(json_obj!(
                    "status_elapsed" => elapsed_fmt.clone(),
                    "status_elapsed_secs" => elapsed.as_secs_f64(),
                )),
                ("Status checked", theme::style::success),
                (" (", theme::style::regular),
                (elapsed_fmt.to_string(), theme::style::elapsed_time),
                (")", theme::style::regular),
            )?;
            Ok(())
        }
    }

    #[trace_instrument(level = "info", skip(theme), fields(full = %full))]
    fn show_status(full: bool, theme: &ThemedUi) -> Result<()> {
        let status = necronux::pkg::GrimoireBindingStatus::introspect()?;

        let stdout = stdout();
        let mut stdout_handle = stdout.lock();

        if status.is_bound {
            result_success_msg!(
                theme,
                &mut stdout_handle,
                None,
                ("Grimoire is currently bound", theme::style::success),
            )?;

            new_line!(&mut stderr())?;

            result_regular_msg!(
                theme,
                &mut stdout_handle,
                None,
                ("GRIMOIRE INFO", theme::style::regular_category_heading),
            )?;

            if let Some(name) = &status.package_name {
                result_regular_msg!(
                    theme,
                    &mut stdout_handle,
                    Some(json_obj!("grimoire_name" => name.clone())),
                    ("Name: ", theme::style::regular_bold),
                    (name.to_string(), theme::style::regular),
                )?;
            }

            if let Some(version) = &status.package_version {
                result_regular_msg!(
                    theme,
                    &mut stdout_handle,
                    Some(json_obj!("grimoire_version" => version.clone())),
                    ("Version: ", theme::style::regular_bold),
                    (version.to_string(), theme::style::regular),
                )?;
            }

            if !full {
                new_line!(&mut stderr())?;
            }
        } else {
            warning_msg!(
                theme,
                &mut stderr(),
                ("No grimoire is currently bound", theme::style::warning),
            )?;
            return Ok(());
        }

        if full {
            stdout_handle.flush()?;
            let pb = task_msg!(
                theme,
                &mut stderr(),
                wants_spinner: true,
                has_steps: false,
                ("Reading grimoire to get full status...", theme::style::progress_task),
            )?;
            let grimoire =
                ValidateSubCmd::validate_grimoire().context("Failed to validate grimoire")?;
            if let Some(pb) = pb {
                pb.finish_and_clear();
            }

            let license = grimoire
                .grimoire_metadata
                .as_ref()
                .and_then(|meta| meta.grimoire_license.as_deref())
                .map(|s| s.to_string())
                .unwrap_or_else(|| utils::missing_field_placeholder("license"));
            result_regular_msg!(
                theme,
                &mut stdout_handle,
                Some(json_obj!("grimoire_license" => license.clone())),
                ("License: ", theme::style::regular_bold),
                (license.to_string(), theme::style::regular),
            )?;

            let std_schema_version = grimoire
                .std_schema_version
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| utils::missing_field_placeholder("schema version"));
            result_regular_msg!(
                theme,
                &mut stdout_handle,
                Some(json_obj!("grimoire_std_schema_version" => std_schema_version.clone())),
                ("Schema Version: ", theme::style::regular_bold),
                (std_schema_version.to_string(), theme::style::regular),
            )?;

            if let Some(path) = &status.grimoire_path {
                let path_str = path.to_string_lossy();
                result_regular_msg!(
                    theme,
                    &mut stdout_handle,
                    Some(json_obj!("bound_path" => path_str.clone())),
                    ("Bound Path: ", theme::style::regular_bold),
                    (path_str.to_string(), theme::style::regular),
                )?;
            }

            regular_msg!(
                theme,
                &mut stderr(),
                ("Grimoire is valid", theme::style::regular_bold),
            )?;

            new_line!(&mut stderr())?;
        }
        Ok(())
    }
}
