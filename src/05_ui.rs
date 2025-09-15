// ==----------------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2024-2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==----------------------------------------------------------------== //

use crate::UiContext;
use necronux::utils::trace_instrument;
use tracing::info;

impl<'u> UiContext<'u> {
    #[trace_instrument(level = "info", name = "UiContext::init", skip(self))]
    pub fn init(&self) {
        info!("Initializing Ui setup...");

        let bool_to_status = |b| if b { "enabled" } else { "disabled" };

        let (colored_stdout_output, colored_stderr_output) = {
            let color_stdout = self.app_ctx.should_use_color_stdout();
            let color_stderr = self.app_ctx.should_use_color_stderr();

            console::set_colors_enabled(color_stdout);
            console::set_colors_enabled_stderr(color_stderr);

            (bool_to_status(color_stdout), bool_to_status(color_stderr))
        };

        let (unicode_stdout_output, unicode_stderr_output) = (
            bool_to_status(self.app_ctx.should_use_unicode_stdout()),
            bool_to_status(self.app_ctx.should_use_unicode_stderr()),
        );

        info!("Colored output: (stdout:{colored_stdout_output}) (stderr:{colored_stderr_output})",);
        info!("Unicode output: (stdout:{unicode_stdout_output}) (stderr:{unicode_stderr_output})",);
    }
}
