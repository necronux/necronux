// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{LogSetup, error::SetLayerError};
use std::result as stdrt;
use tracing::Subscriber;
use tracing_subscriber::{Layer, fmt::format::FmtSpan, registry::LookupSpan};

impl LogSetup {
    pub fn json_fmt_layer<S>(&self) -> stdrt::Result<impl Layer<S>, SetLayerError>
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        let format = super::json_fmt_format();

        let cache_dir_path = necronux_utils::paths::cache_necronux_path()?;
        necronux_utils::fs::create_dir_all(&cache_dir_path, "necronux cache dir")?;

        let file_path = necronux_utils::paths::necronux_log_file_path()?;
        let file = necronux_utils::fs::create_file(&file_path, "necronux log file")?;

        Ok(tracing_subscriber::fmt::layer()
            .event_format(format)
            .with_ansi(false)
            .with_span_events(FmtSpan::FULL)
            .with_writer(file))
    }
}
