// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::style;
use once_cell::sync::Lazy;

pub static TASK_SPINNER_TICKS: Lazy<Vec<String>> = Lazy::new(|| {
    let runes = ["ᚾ", "ᛖ", "ᚲ", "ᚱ", "ᛟ", "ᚾ", "ᚢ", "ᛉ"];
    runes
        .iter()
        .map(|r| format!("❰ {} ❱", style::progress_task(r)))
        .collect()
});
pub static STEP_SPINNER_TICKS: Lazy<Vec<String>> = Lazy::new(|| {
    let runes = ["ᚾ", "ᛖ", "ᚲ", "ᚱ", "ᛟ", "ᚾ", "ᚢ", "ᛉ"];
    runes
        .iter()
        .map(|r| format!("❰ {} ❱", style::progress_step(r)))
        .collect()
});

pub fn task_spinner_ticks() -> Vec<String> {
    TASK_SPINNER_TICKS.clone()
}
pub fn step_spinner_ticks() -> Vec<String> {
    STEP_SPINNER_TICKS.clone()
}

pub fn regular() -> String {
    format!("❰ {} ❱", style::regular("ᛉ"))
}

pub fn task() -> String {
    format!("❰ {} ❱", style::progress_task("ᛉ"))
}

pub fn step() -> String {
    format!("❰ {} ❱", style::progress_step("ᛉ"))
}

pub fn success() -> String {
    format!("❰ {} ❱", style::success("ᛟ"))
}

pub fn warning() -> String {
    format!("❰ {} ❱", style::warning("ᚦ"))
}
