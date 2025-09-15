// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::theme::ThemedUi;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

impl<'t> ThemedUi<'t> {
    pub fn progress_task_msg_spinner(&self, body: String) -> Result<ProgressBar> {
        self.new_spinner(body, self.progress_task_msg_prefix_with_spinner_ticks())
    }
    pub fn progress_step_msg_spinner(&self, body: String) -> Result<ProgressBar> {
        self.new_spinner(body, self.progress_step_msg_prefix_with_spinner_ticks())
    }

    fn new_spinner(&self, body: String, ticks: &'static [String]) -> Result<ProgressBar> {
        let pb = ProgressBar::new_spinner();
        let tick_refs: Vec<&str> = ticks.iter().map(|s| s.as_str()).collect();
        pb.set_style(ProgressStyle::with_template("{spinner} {msg}")?.tick_strings(&tick_refs));
        pb.enable_steady_tick(Duration::from_millis(100));

        pb.set_message(body);

        Ok(pb)
    }
}
