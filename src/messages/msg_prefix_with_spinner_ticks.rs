// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::theme::{self, ThemedUi};
use once_cell::sync::Lazy;

impl<'t> ThemedUi<'t> {
    pub fn progress_task_msg_prefix_with_spinner_ticks(&self) -> &'static Vec<String> {
        if self.0.app_ctx.should_use_unicode_stderr() {
            &PROGRESS_TASK_SPINNER_TICKS
        } else {
            &PROGRESS_TASK_SPINNER_TICKS_NO_UNICODE
        }
    }

    pub fn progress_step_msg_prefix_with_spinner_ticks(&self) -> &'static Vec<String> {
        if self.0.app_ctx.should_use_unicode_stderr() {
            &PROGRESS_STEP_SPINNER_TICKS
        } else {
            &PROGRESS_STEP_SPINNER_TICKS_NO_UNICODE
        }
    }
}

fn format_spinner_ticks<F, S>(unicode: bool, style_fn: F) -> Vec<String>
where
    F: Fn(&'static str) -> S,
    S: std::fmt::Display,
{
    theme::rune::SPINNER_RUNE_TICKS_WITH_FALLBACK
        .iter()
        .map(|r| {
            let u = if unicode { r.rune } else { r.fallback };
            let content = style_fn(u);

            if unicode {
                format!("❰ {content} ❱")
            } else {
                format!("< {content} >")
            }
        })
        .collect()
}

static PROGRESS_TASK_SPINNER_TICKS: Lazy<Vec<String>> =
    Lazy::new(|| format_spinner_ticks(true, theme::style::progress_task));

static PROGRESS_STEP_SPINNER_TICKS: Lazy<Vec<String>> =
    Lazy::new(|| format_spinner_ticks(true, theme::style::progress_step));

static PROGRESS_TASK_SPINNER_TICKS_NO_UNICODE: Lazy<Vec<String>> =
    Lazy::new(|| format_spinner_ticks(false, theme::style::progress_task));

static PROGRESS_STEP_SPINNER_TICKS_NO_UNICODE: Lazy<Vec<String>> =
    Lazy::new(|| format_spinner_ticks(false, theme::style::progress_step));
