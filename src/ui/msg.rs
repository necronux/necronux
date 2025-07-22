// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::symbol;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::{borrow::Cow, time::Duration};

pub fn task_spinner_or_msg(
    has_steps: bool,
    progress: bool,
    msg: impl Into<Cow<'static, str>> + std::fmt::Display,
) -> Result<Option<ProgressBar>> {
    if progress && has_steps {
        task_msg(msg)?;
        Ok(None)
    } else {
        let pb = ProgressBar::new_spinner();

        let ticks = symbol::task_spinner_ticks();
        let tick_refs: Vec<&str> = ticks.iter().map(|s| s.as_str()).collect();

        pb.set_style(ProgressStyle::with_template("{spinner} {msg}")?.tick_strings(&tick_refs));
        pb.enable_steady_tick(Duration::from_millis(100));

        pb.set_message(msg);
        Ok(Some(pb))
    }
}

pub fn step_spinner(
    progress: bool,
    msg: impl Into<Cow<'static, str>> + std::fmt::Display,
) -> Result<Option<ProgressBar>> {
    if progress {
        let pb = ProgressBar::new_spinner();

        let ticks = symbol::step_spinner_ticks();
        let tick_refs: Vec<&str> = ticks.iter().map(|s| s.as_str()).collect();

        pb.set_style(ProgressStyle::with_template("{spinner} {msg}")?.tick_strings(&tick_refs));
        pb.enable_steady_tick(Duration::from_millis(100));

        pb.set_message(msg);
        Ok(Some(pb))
    } else {
        Ok(None)
    }
}

fn task_msg(msg: impl Into<Cow<'static, str>> + std::fmt::Display) -> Result<()> {
    println!("{} {}", symbol::task(), &msg);
    Ok(())
}

pub fn step_msg(
    progress: bool,
    msg: impl Into<Cow<'static, str>> + std::fmt::Display,
) -> Result<()> {
    if progress {
        println!("{} {}", symbol::step(), &msg);
    }
    Ok(())
}
