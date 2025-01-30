// ==----------------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2024-2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==----------------------------------------------------------------== //

use color_eyre::eyre::Result;
use flexi_logger::{
    style, AdaptiveFormat, DeferredNow, LogSpecBuilder, LogSpecification, Logger, LoggerHandle,
    WriteMode, TS_DASHES_BLANK_COLONS_DOT_BLANK,
};
use log::{debug, LevelFilter, Record};

pub fn init_logger(
    default_log_level: Option<LevelFilter>,
) -> Result<(LogSpecBuilder, LoggerHandle, LevelFilter)> {
    // flexi_logger builder
    let mut logger_builder = LogSpecification::builder();

    // Set default log level.
    let default_log_level = default_log_level.unwrap_or_else(|| {
        if cfg!(debug_assertions) {
            LevelFilter::Debug
        } else {
            LevelFilter::Warn
        }
    });
    logger_builder.default(default_log_level);

    let logger = Logger::with(logger_builder.build())
        .log_to_stderr()
        .adaptive_format_for_stderr(AdaptiveFormat::Custom(
            custom_noncolored_log_format,
            custom_colored_log_format,
        ))
        .set_palette("1;3;2;4;6".to_string())
        .write_mode(WriteMode::BufferAndFlush)
        .start()?;

    debug!(
        "Logger initialized with defaults, log level set to {}.",
        default_log_level
    );

    // Set max log level.
    if cfg!(debug_assertions) {
        log::set_max_level(LevelFilter::Trace);
    } else {
        log::set_max_level(LevelFilter::Info);
    }
    let max_log_level = log::max_level();
    debug!(
        "Max log level set to {} for the default log config.",
        max_log_level
    );

    Ok((logger_builder, logger, max_log_level))
}

fn custom_colored_log_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let level = record.level();

    write!(
        w,
        "[{} {}] {}",
        // Plain (non-colored) timestamp.
        now.format(TS_DASHES_BLANK_COLONS_DOT_BLANK),
        // Colored log level based on severity.
        style(level).paint(record.level().to_string()),
        // Plain (non-colored) log message.
        record.args(),
    )?;

    Ok(())
}

fn custom_noncolored_log_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "[{} {}] {}",
        // Plain (non-colored) timestamp.
        now.format(TS_DASHES_BLANK_COLONS_DOT_BLANK),
        // Plain (non-colored) log level.
        record.level(),
        // Plain (non-colored) log message.
        record.args(),
    )?;

    Ok(())
}
