// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{StatusSubCmd, ValidateHandler, ui};
use anyhow::{Context, Result};

pub struct StatusHandler {
    full: bool,
}

impl StatusHandler {
    pub fn new(subcmd: &StatusSubCmd) -> Self {
        Self { full: subcmd.full }
    }

    pub fn handle(self) -> Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("handle_status").entered();

        let t0 = std::time::Instant::now();

        Self::show_status(self.full).context("Failed to show status")?;

        let elapsed = t0.elapsed();
        let elapsed_fmt = ui::format_duration(elapsed);

        println!(
            "{} {} ({})",
            ui::symbol::success(),
            ui::style::success("Status checked"),
            ui::style::elapsed_time(&elapsed_fmt),
        );
        Ok(())
    }

    fn show_status(full: bool) -> Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("show_status").entered();

        let status = necronux_core::pkg::GrimoireBindingStatus::introspect()?;

        if status.is_bound {
            println!(
                "{} {}",
                ui::symbol::success(),
                ui::style::success("Grimoire is currently bound")
            );

            println!();

            println!(
                "❰ {} ❱",
                ui::style::regular_category_heading("GRIMOIRE INFO")
            );

            if let Some(name) = &status.package_name {
                println!(
                    "{} {} {}",
                    ui::symbol::regular(),
                    ui::style::regular("Name:").bold(),
                    name
                );
            }

            if let Some(version) = &status.package_version {
                println!(
                    "{} {} {}",
                    ui::symbol::regular(),
                    ui::style::regular("Version:").bold(),
                    version
                );
            }

            if !full {
                println!();
            }
        } else {
            println!(
                "{} {}",
                ui::symbol::warning(),
                ui::style::warning("No grimoire is currently bound"),
            );
            return Ok(());
        }

        if full {
            let pb = ui::msg::task_spinner_or_msg(
                false,
                false,
                ui::style::progress_task("Reading grimoire to get full status...").to_string(),
            )?;
            let grimoire =
                ValidateHandler::validate_grimoire().context("Failed to validate grimoire")?;
            if let Some(pb) = pb {
                pb.finish_and_clear();
            }

            let fallback = ui::no("license");
            let license = grimoire
                .grimoire_metadata
                .as_ref()
                .and_then(|meta| meta.grimoire_license.as_deref())
                .unwrap_or(&fallback);
            println!(
                "{} {} {}",
                ui::symbol::regular(),
                ui::style::regular("License:").bold(),
                license
            );

            let fallback = ui::no("schema version");
            let std_schema_version = grimoire.std_schema_version.as_ref().unwrap_or(&fallback);
            println!(
                "{} {} {}",
                ui::symbol::regular(),
                ui::style::regular("Schema Version:").bold(),
                std_schema_version
            );

            if let Some(path) = &status.grimoire_path {
                println!(
                    "{} {} {}",
                    ui::symbol::regular(),
                    ui::style::regular("Bound Path:").bold(),
                    path.display()
                );
            }

            println!(
                "{} {}",
                ui::symbol::regular(),
                ui::style::regular("Grimoire is valid").bold()
            );

            println!();
        }
        Ok(())
    }
}
