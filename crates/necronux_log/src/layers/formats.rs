// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use tracing_subscriber::fmt::format::{Compact, Format, Json};

#[cfg(debug_assertions)]
use tracing_subscriber::fmt::format::Pretty;
#[cfg(debug_assertions)]
pub fn stderr_fmt_extended_format() -> Format<Pretty> {
    tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_source_location(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .pretty()
}

pub fn stderr_fmt_format() -> Format<Compact> {
    let base = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_source_location(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .compact();

    #[cfg(debug_assertions)]
    let format = base.with_target(true);

    #[cfg(not(debug_assertions))]
    let format = base.with_target(false);

    format
}

pub fn json_fmt_format() -> Format<Json> {
    let base = tracing_subscriber::fmt::format::json()
        .with_level(true)
        .with_source_location(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_current_span(true)
        .with_span_list(true)
        .flatten_event(false);

    #[cfg(debug_assertions)]
    let format = base.with_target(true);

    #[cfg(not(debug_assertions))]
    let format = base.with_target(false);

    format
}
