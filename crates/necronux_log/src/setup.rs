// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use indicatif::ProgressState;
use std::time::Duration;
use tracing::{info, level_filters::LevelFilter};
use tracing_indicatif::{IndicatifLayer, style::ProgressStyle};
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

        let style = ProgressStyle::with_template(
            "{color_start}{span_child_prefix}{span_fields} -- {span_name} {wide_msg} {elapsed_subsec}{color_end}",
        )?;

        let indicatif_layer = IndicatifLayer::new()
            .with_progress_style(
                style
                    .with_key("elapsed_subsec", Self::elapsed_subsec)
                    .with_key(
                        "color_start",
                        |state: &ProgressState, writer: &mut dyn std::fmt::Write| {
                            let elapsed = state.elapsed();
                            if elapsed > Duration::from_secs(8) {
                                // Red
                                let _ = write!(writer, "\x1b[{}m", 1 + 30);
                            } else if elapsed > Duration::from_secs(4) {
                                // Yellow
                                let _ = write!(writer, "\x1b[{}m", 3 + 30);
                            }
                        },
                    )
                    .with_key(
                        "color_end",
                        |state: &ProgressState, writer: &mut dyn std::fmt::Write| {
                            if state.elapsed() > Duration::from_secs(4) {
                                let _ = write!(writer, "\x1b[0m");
                            }
                        },
                    ),
            )
            .with_span_child_prefix_symbol("↳ ")
            .with_span_child_prefix_indent(" ");

        let fmt_layer = tracing_subscriber::fmt::layer()
            .event_format(format)
            .with_ansi(true)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
            .with_writer(indicatif_layer.get_stderr_writer());

        let filter_layer = self.level;

        let subscriber = tracing_subscriber::registry()
            .with(fmt_layer)
            .with(filter_layer)
            .with(indicatif_layer);

        tracing::subscriber::set_global_default(subscriber)?;

        tracing_log::LogTracer::init()?;

        info!("Logging initialized; requested log level: {}", self.level);
        Ok(())
    }

    fn elapsed_subsec(state: &ProgressState, writer: &mut dyn std::fmt::Write) {
        let seconds = state.elapsed().as_secs();
        let sub_seconds = (state.elapsed().as_millis() % 1000) / 100;
        let _ = writer.write_str(&format!("{seconds}.{sub_seconds}s"));
    }
}
