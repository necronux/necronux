// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use console::{StyledObject, style};

pub fn progress_task(label: &str) -> StyledObject<&str> {
    style(label).blue().bold()
}
pub fn progress_step(label: &str) -> StyledObject<&str> {
    style(label).white()
}

pub fn warning(label: &str) -> StyledObject<&str> {
    style(label).yellow().bold()
}
pub fn success(label: &str) -> StyledObject<&str> {
    style(label).green().bold()
}

pub fn regular(label: &str) -> StyledObject<&str> {
    style(label).white()
}
pub fn regular_bold(label: &str) -> StyledObject<&str> {
    regular(label).bold()
}

pub fn regular_category_heading(label: &str) -> StyledObject<&str> {
    style(label).white().bold()
}

pub fn elapsed_time(label: &str) -> StyledObject<&str> {
    style(label).dim()
}

// BASIC

pub fn underlined(label: &str) -> StyledObject<&str> {
    style(label).underlined()
}

pub fn bold(label: &str) -> StyledObject<&str> {
    style(label).bold()
}

pub fn dim(label: &str) -> StyledObject<&str> {
    style(label).dim()
}
