// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::Result;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;

pub struct LogSetup {
    pub level: tracing::level_filters::LevelFilter,
    pub color: bool,
    #[cfg(debug_assertions)]
    pub extended: bool,
}

impl LogSetup {
    pub fn with_default_level(no_color: bool, #[cfg(debug_assertions)] extended: bool) -> Self {
        Self {
            level: tracing::level_filters::LevelFilter::WARN,
            color: !no_color,
            #[cfg(debug_assertions)]
            extended,
        }
    }

    #[cfg(feature = "cli_arg_log")]
    pub fn from_cli_or_with_default_level<L>(
        verbosity: clap_verbosity_flag::Verbosity<L>,
        no_color: bool,
        #[cfg(debug_assertions)] extended: bool,
    ) -> Self
    where
        L: clap_verbosity_flag::LogLevel,
    {
        if verbosity.is_present() {
            Self {
                level: verbosity.into(),
                color: !no_color,
                #[cfg(debug_assertions)]
                extended,
            }
        } else {
            Self::with_default_level(
                no_color,
                #[cfg(debug_assertions)]
                extended,
            )
        }
    }

    pub fn init(self) -> Result<()> {
        #[cfg(debug_assertions)]
        return self.init_debug();

        #[cfg(not(debug_assertions))]
        return self.init_release();
    }

    #[cfg(debug_assertions)]
    fn init_debug(self) -> Result<()> {
        let format = if self.extended {
            tracing_subscriber::fmt::format()
                .with_level(true)
                .with_target(true)
                .with_source_location(true)
                .with_thread_ids(true)
                .with_thread_names(true)
        } else {
            tracing_subscriber::fmt::format()
                .with_level(true)
                .with_target(false)
                .with_source_location(false)
                .with_thread_ids(false)
                .with_thread_names(false)
        };

        let fmt_layer = tracing_subscriber::fmt::layer()
            .event_format(format)
            .with_ansi(self.color)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL);

        self.init_with_layer(fmt_layer)
    }

    #[cfg(not(debug_assertions))]
    fn init_release(self) -> Result<()> {
        let format = tracing_subscriber::fmt::format()
            .with_level(true)
            .with_target(false)
            .with_source_location(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .compact();

        let fmt_layer = tracing_subscriber::fmt::layer()
            .event_format(format)
            .with_ansi(self.color)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL);

        self.init_with_layer(fmt_layer)
    }

    fn init_with_layer(
        self,
        fmt_layer: impl tracing_subscriber::Layer<tracing_subscriber::Registry> + Send + Sync + 'static,
    ) -> Result<()> {
        let filter_layer = self.level;

        let subscriber = tracing_subscriber::registry()
            .with(fmt_layer)
            .with(filter_layer);

        tracing::subscriber::set_global_default(subscriber)?;

        tracing_log::LogTracer::init()?;

        info!("Logging initialized; requested log level: {}", self.level);
        Ok(())
    }
}
