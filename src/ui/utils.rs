// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use std::time::Duration;

pub fn format_duration(d: Duration) -> String {
    if d < Duration::from_millis(1) {
        "<1ms".to_string()
    } else if d < Duration::from_secs(1) {
        format!("{}ms", d.as_millis())
    } else if d < Duration::from_secs(60) {
        format!("{:.1}s", d.as_secs_f32())
    } else {
        let mins = d.as_secs() / 60;
        let secs = d.as_secs() % 60;
        match secs {
            0 => format!("{mins}min"),
            _ => format!("{mins}min {secs}s"),
        }
    }
}

pub fn no(field: &str) -> String {
    format!("<no {field}>")
}
