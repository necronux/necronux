// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use anyhow::Result;
use tracing::info;

pub struct UiSetup {
    color: bool,
}

impl UiSetup {
    pub fn new(no_color: bool) -> Self {
        Self { color: !no_color }
    }

    pub fn init(self) -> Result<()> {
        info!("Initializing Ui setup...");

        console::set_colors_enabled(self.color);
        console::set_colors_enabled_stderr(self.color);

        info!(
            "Console color output {}",
            if self.color { "enabled" } else { "disabled" }
        );

        Ok(())
    }
}
