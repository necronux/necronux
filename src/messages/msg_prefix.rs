// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::theme::{self, ThemedUi, rune::NonSpinnerRune};

impl<'t> ThemedUi<'t> {
    pub fn progress_task_msg_prefix(&self) -> String {
        self.format_msg_prefix(
            &theme::rune::PROGRESS_RUNE_WITH_FALLBACK,
            theme::style::progress_task,
        )
    }
    pub fn progress_step_msg_prefix(&self) -> String {
        self.format_msg_prefix(
            &theme::rune::PROGRESS_RUNE_WITH_FALLBACK,
            theme::style::progress_step,
        )
    }

    pub fn warning_msg_prefix(&self) -> String {
        self.format_msg_prefix(
            &theme::rune::WARNING_RUNE_WITH_FALLBACK,
            theme::style::warning,
        )
    }
    pub fn success_msg_prefix(&self) -> String {
        self.format_msg_prefix(
            &theme::rune::SUCCESS_RUNE_WITH_FALLBACK,
            theme::style::success,
        )
    }

    pub fn regular_msg_prefix(&self) -> String {
        self.format_msg_prefix(
            &theme::rune::REGULAR_RUNE_WITH_FALLBACK,
            theme::style::regular,
        )
    }

    fn format_msg_prefix<F, S>(&self, r: &NonSpinnerRune, style_fn: F) -> String
    where
        F: Fn(&'static str) -> S,
        S: std::fmt::Display,
    {
        let unicode = self.0.app_ctx.should_use_unicode_stdout();
        let u = if unicode { r.rune } else { r.fallback };
        let content = style_fn(u);

        if unicode {
            format!("❰ {content} ❱")
        } else {
            format!("< {content} >")
        }
    }
}
