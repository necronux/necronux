use crate::controller::Cli;
use color_eyre::eyre::Result;
use flexi_logger::{
    style, AdaptiveFormat, DeferredNow, LogSpecBuilder, LogSpecification, Logger, LoggerHandle,
    WriteMode, TS_DASHES_BLANK_COLONS_DOT_BLANK,
};
use log::{debug, LevelFilter, Record};
use std::env;

pub fn init_logger() -> Result<(LogSpecBuilder, LoggerHandle, LevelFilter)> {
    // flexi_logger builder
    let mut logger_builder = LogSpecification::builder();

    // Set default log level.
    let default_log_level = if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Warn
    };
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

pub fn override_logger(
    logger_builder: &mut LogSpecBuilder,
    logger: &LoggerHandle,
    max_log_level: LevelFilter,
    cli: &Cli,
) -> Result<()> {
    // The order of precedence is:
    // Default < Necronux Bundle File < Environment Variable < CLI Verbosity Flag
    debug!("Loading log level setting from various sources...");

    let mut log_level_updated = false;

    // Override log level from the necronux bundle file.
    //
    //  TODO
    //

    // Override log level from the environment variable.
    if let Ok(env_log_level_str) = env::var("NECRONUX_LOG_LEVEL") {
        match env_log_level_str.as_str() {
            "TRACE" => Some(LevelFilter::Trace),
            "DEBUG" => Some(LevelFilter::Debug),
            "INFO" => Some(LevelFilter::Info),
            "WARN" => Some(LevelFilter::Warn),
            "ERROR" => Some(LevelFilter::Error),
            "OFF" => Some(LevelFilter::Off),
            _ => None,
        }
        .map_or_else(
            || {
                debug!(
                    "Invalid log level {} in NECRONUX_LOG_LEVEL. Ignoring...",
                    env_log_level_str
                );
            },
            |env_log_level| {
                debug!(
                    "Overriding log level to {} from environment variable.",
                    env_log_level
                );
                logger_builder.default(env_log_level);
                log_level_updated = true;
            },
        );
    }

    // Override log level from the cli verbosity flag.
    if cli.verbose.is_present() {
        let cli_log_level = cli.verbose.log_level_filter();
        debug!(
            "Overriding log level to {} from cli verbosity flag.",
            cli_log_level
        );
        logger_builder.default(cli_log_level);
        log_level_updated = true;
    }

    // If there were any updates, apply the new specification to the logger.
    if log_level_updated {
        let new_log_level = logger_builder.build();
        logger.set_new_spec(new_log_level.clone());

        let new_log_level_str = new_log_level.to_string().to_uppercase();
        debug!("Log level updated to {}.", new_log_level_str);

        // Set max log level again.
        log::set_max_level(max_log_level);
        let overriden_max_log_level = log::max_level();
        debug!(
            "Max log level set to {} for the overriden log config.",
            overriden_max_log_level
        );

        debug!("Log level setting loaded from various sources and merged.");
    } else {
        debug!("No log level updates applied, no overrides found in various sources.");
    }

    Ok(())
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
