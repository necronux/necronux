// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::{InitializeError, LogError};
use std::result as stdrt;
use tracing::info;
use tracing_subscriber::{Layer, layer::SubscriberExt};

pub struct LogSetup {
    pub global_level_filter: tracing::level_filters::LevelFilter,
    pub color: bool,
    pub unicode: bool,
    pub stderr_layer: Option<StdErrLayer>,
    pub log_file_json_layer: bool,
    #[cfg(debug_assertions)]
    pub chrome_layer: bool,
}

pub enum StdErrLayer {
    Fmt,
    #[cfg(debug_assertions)]
    FmtExtended,
    #[cfg(debug_assertions)]
    FancyTree,
}

impl Default for LogSetup {
    fn default() -> Self {
        Self {
            global_level_filter: tracing::level_filters::LevelFilter::WARN,
            color: true,
            unicode: true,
            stderr_layer: Some(StdErrLayer::Fmt),
            log_file_json_layer: true,
            #[cfg(debug_assertions)]
            chrome_layer: true,
        }
    }
}

impl LogSetup {
    pub fn with_level(mut self, level: tracing::level_filters::LevelFilter) -> Self {
        self.global_level_filter = level;
        self
    }

    pub fn with_color(mut self, color_enabled: bool) -> Self {
        self.color = color_enabled;
        self
    }

    pub fn with_unicode(mut self, unicode_enabled: bool) -> Self {
        self.unicode = unicode_enabled;
        self
    }

    pub fn with_stderr_layer(mut self, layer: Option<StdErrLayer>) -> Self {
        self.stderr_layer = layer;
        self
    }

    pub fn with_log_file_json_layer(mut self, layer: bool) -> Self {
        self.log_file_json_layer = layer;
        self
    }

    #[cfg(debug_assertions)]
    pub fn with_chrome_layer(mut self, layer: bool) -> Self {
        self.chrome_layer = layer;
        self
    }

    pub fn init(self) -> stdrt::Result<(), LogError> {
        fn init_inner(
            stp: LogSetup,
        ) -> stdrt::Result<tracing::level_filters::LevelFilter, InitializeError> {
            let global_level_filter = stp.global_level_filter;

            let stderr_layer = match stp.stderr_layer {
                Some(StdErrLayer::Fmt) => Some(stp.stderr_fmt_layer().boxed()),
                #[cfg(debug_assertions)]
                Some(StdErrLayer::FmtExtended) => Some(stp.stderr_fmt_extended_layer().boxed()),
                #[cfg(debug_assertions)]
                Some(StdErrLayer::FancyTree) => Some(stp.stderr_fancytree_layer().boxed()),
                None => None,
            };

            let log_file_json_layer = if stp.log_file_json_layer {
                let layer = stp.json_fmt_layer()?;
                Some(layer)
            } else {
                None
            };

            #[cfg(debug_assertions)]
            let chrome_layer = if stp.chrome_layer {
                let layer = stp.chrome_layer_with_guard()?;
                Some(layer)
            } else {
                None
            };

            let subscriber = tracing_subscriber::registry()
                .with(global_level_filter)
                .with(stderr_layer)
                .with(log_file_json_layer);

            #[cfg(debug_assertions)]
            let subscriber = subscriber.with(chrome_layer);

            tracing::subscriber::set_global_default(subscriber)?;

            tracing_log::LogTracer::init()?;

            Ok(global_level_filter)
        }

        let global_level_filter =
            init_inner(self).map_err(|e| LogError::InitializeError { source: e })?;

        info!(
            requested_log_level = %global_level_filter,
            "Logging initialized"
        );
        Ok(())
    }
}
