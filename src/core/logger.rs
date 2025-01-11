use color_eyre::eyre::Result;
use config::Config;
use flexi_logger::{
    style, AdaptiveFormat, DeferredNow, LogSpecBuilder, LogSpecification, Logger, LoggerHandle,
    WriteMode, TS_DASHES_BLANK_COLONS_DOT_BLANK,
};
use log::{debug, LevelFilter, Record};

pub fn init_logger() -> Result<(LogSpecBuilder, LevelFilter, LoggerHandle)> {
    let mut builder = LogSpecification::builder();

    let default_log_level = if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Warn
    };
    builder.default(default_log_level);

    let logger = Logger::with(builder.build())
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

    // Set max log level filter.
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

    Ok((builder, default_log_level, logger))
}

pub fn override_logger(
    builder: &mut LogSpecBuilder,
    logger: &LoggerHandle,
    merged_config: &Config,
) -> Result<()> {
    // Retrieve the log level from merged config.
    let log_level: String = merged_config.get("logging.level")?;
    let new_log_level = match log_level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "off" => LevelFilter::Off,
        // Default to warn if level is invalid or not found.
        _ => LevelFilter::Warn,
    };

    // Modify the builder to set the new log level.
    builder.default(new_log_level);

    // Apply the new specification to the logger.
    logger.set_new_spec(builder.build());

    // Set max log level filter again.
    if cfg!(debug_assertions) {
        log::set_max_level(LevelFilter::Trace);
    } else {
        log::set_max_level(LevelFilter::Info);
    }
    debug!("Log level updated to {}.", new_log_level);
    let max_log_level = log::max_level();
    debug!(
        "Max log level set to {} for the overriden log config.",
        max_log_level
    );

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
