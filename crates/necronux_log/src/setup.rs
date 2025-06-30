// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::layer::SubscriberExt;

pub struct LogSetup {
    pub level: LevelFilter,
}

impl Default for LogSetup {
    fn default() -> Self {
        Self {
            level: (LevelFilter::WARN),
        }
    }
}

impl LogSetup {
    #[cfg(feature = "cli_arg_log")]
    pub fn from_cli_or_default<L>(verbosity: clap_verbosity_flag::Verbosity<L>) -> Self
    where
        L: clap_verbosity_flag::LogLevel,
    {
        if verbosity.is_present() {
            Self::from_cli_verbosity(verbosity)
        } else {
            Self::default()
        }
    }

    #[cfg(feature = "cli_arg_log")]
    fn from_cli_verbosity<L>(verbosity: clap_verbosity_flag::Verbosity<L>) -> Self
    where
        L: clap_verbosity_flag::LogLevel,
    {
        Self {
            level: verbosity.into(),
        }
    }

    pub fn init(self) -> anyhow::Result<()> {
        #[cfg(debug_assertions)]
        let format = tracing_subscriber::fmt::format()
            .with_level(true)
            .with_target(true)
            .with_source_location(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .pretty();

        #[cfg(not(debug_assertions))]
        let format = tracing_subscriber::fmt::format()
            .with_level(true)
            .with_target(false)
            .with_source_location(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .compact();

        let fmt_layer = tracing_subscriber::fmt::layer()
            .event_format(format)
            .with_ansi(true)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL);

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
