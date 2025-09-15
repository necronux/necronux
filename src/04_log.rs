// ==----------------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2024-2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==----------------------------------------------------------------== //

use crate::LogContext;
use anyhow::Result;

impl<'l> LogContext<'l> {
    pub fn init_if_enabled(&self) -> Result<()> {
        if !self.app_ctx.config.logging {
            return Ok(());
        }
        self.init()
    }

    pub fn init(&self) -> Result<()> {
        #[cfg(not(debug_assertions))]
        let log_format_stderr = Some(necronux::log::StdErrLayer::Fmt);

        #[cfg(debug_assertions)]
        let log_format_stderr = self.app_ctx.config.log_output_format.into();

        let color = self.app_ctx.should_use_color_stderr();
        let unicode = self.app_ctx.should_use_unicode_stderr();

        let log_setup = necronux::log::LogSetup::default()
            .with_level(self.app_ctx.config.verbosity.into())
            .with_color(color)
            .with_unicode(unicode)
            .with_stderr_layer(log_format_stderr)
            .with_log_file_json_layer(true);

        #[cfg(debug_assertions)]
        let log_setup = log_setup.with_chrome_layer(true);

        log_setup.init()?;
        Ok(())
    }
}
