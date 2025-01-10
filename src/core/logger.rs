use color_eyre::eyre::Result;
use flexi_logger::{
    style, AdaptiveFormat, DeferredNow, Logger, WriteMode, TS_DASHES_BLANK_COLONS_DOT_BLANK,
};
use log::{debug, Record};

pub fn init_logger() -> Result<()> {
    // Initializes the logger with default settings.
    init_logger_with_defaults()?;

    Ok(())
}

pub fn override_logger() -> Result<()> {
    // Overrides the logger.
    override_logger_with_config()?;

    Ok(())
}

fn init_logger_with_defaults() -> Result<()> {
    Logger::try_with_str("trace")?
        .log_to_stderr()
        .adaptive_format_for_stderr(AdaptiveFormat::Custom(
            custom_noncolored_log_format,
            custom_colored_log_format,
        ))
        .set_palette("1;3;2;4;6".to_string())
        .write_mode(WriteMode::BufferAndFlush)
        .start()?;

    debug!("Logger initialized with defaults.");

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

fn override_logger_with_config() -> Result<()> {
    Ok(())
}
