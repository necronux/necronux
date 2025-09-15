// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{CliProgressLevel, theme::ThemedUi};
use anyhow::Result;
use indicatif::ProgressBar;

pub type SpinnerFn<'t> = fn(&ThemedUi<'t>, String) -> Result<ProgressBar>;

pub enum MsgKind {
    Task {
        wants_spinner: bool,
        has_steps: bool,
    },
    Step {
        wants_spinner: bool,
    },
    Warning,
    Success,
    Regular,
    StdOutSuccess,
    StdOutRegular,
}

impl MsgKind {
    pub fn prefix<'t>(&self, t: &ThemedUi<'t>) -> String {
        match *self {
            MsgKind::Task { .. } => t.progress_task_msg_prefix(),
            MsgKind::Step { .. } => t.progress_step_msg_prefix(),
            MsgKind::Warning => t.warning_msg_prefix(),
            MsgKind::Success => t.success_msg_prefix(),
            MsgKind::Regular => t.regular_msg_prefix(),
            MsgKind::StdOutSuccess => t.success_msg_prefix(),
            MsgKind::StdOutRegular => t.regular_msg_prefix(),
        }
    }

    pub fn spinner_fn<'t>(&self, t: &ThemedUi<'t>) -> Option<SpinnerFn<'t>> {
        match *self {
            MsgKind::Task {
                wants_spinner,
                has_steps,
            } => {
                let detailed = matches!(
                    t.0.app_ctx.config.progress_level,
                    CliProgressLevel::Detailed
                );
                let disable = detailed && has_steps;
                if wants_spinner && !disable {
                    Some(ThemedUi::progress_task_msg_spinner)
                } else {
                    None
                }
            }
            MsgKind::Step { wants_spinner } => {
                let detailed = matches!(
                    t.0.app_ctx.config.progress_level,
                    CliProgressLevel::Detailed
                );
                if wants_spinner && detailed {
                    Some(ThemedUi::progress_step_msg_spinner)
                } else {
                    None
                }
            }
            MsgKind::Warning
            | MsgKind::Success
            | MsgKind::Regular
            | MsgKind::StdOutSuccess
            | MsgKind::StdOutRegular => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            MsgKind::Task { .. } => "task",
            MsgKind::Step { .. } => "step",
            MsgKind::Warning => "warning",

            MsgKind::Success => "success",
            MsgKind::Regular => "regular",

            MsgKind::StdOutSuccess => "stdout_success",
            MsgKind::StdOutRegular => "stdout_regular",
        }
    }

    pub fn is_stdout_msg_kind(&self) -> bool {
        matches!(self, MsgKind::StdOutSuccess | MsgKind::StdOutRegular)
    }
}
