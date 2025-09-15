// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::LogSetup;
use tracing::Subscriber;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{Layer, registry::LookupSpan};

impl LogSetup {
    pub fn stderr_fmt_layer<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        let format = super::stderr_fmt_format();

        tracing_subscriber::fmt::layer()
            .event_format(format)
            .with_ansi(self.color)
            .with_span_events(FmtSpan::FULL)
            .with_writer(std::io::stderr)
    }

    #[cfg(debug_assertions)]
    pub fn stderr_fmt_extended_layer<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        let format = super::stderr_fmt_extended_format();

        tracing_subscriber::fmt::layer()
            .event_format(format)
            .with_ansi(self.color)
            .with_span_events(FmtSpan::FULL)
            .with_writer(std::io::stderr)
    }
}
