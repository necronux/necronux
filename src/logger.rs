use crate::controller::Cli;
use color_eyre::eyre::Result;
use env_logger::{Builder, Env};
use log::{debug, LevelFilter};

pub fn init_logger(cli: &Cli) -> Result<()> {
    let mut builder = Builder::new();

    let log_level_env_var = Env::default()
        .filter("NECRONUX_LOG_LEVEL")
        .write_style("NECRONUX_LOG_STYLE");

    builder.filter_level(LevelFilter::Warn);
    builder.parse_env(log_level_env_var);
    if cli.verbose.is_present() {
        builder.filter_level(cli.verbose.log_level_filter());
    }

    builder
        .format_target(false)
        .format_timestamp_millis()
        .init();

    #[cfg(debug_assertions)]
    log::set_max_level(LevelFilter::Trace);
    #[cfg(not(debug_assertions))]
    log::set_max_level(LevelFilter::Info);

    debug!("Initialized env_logger builder");
    debug!("Set max log level filter");

    Ok(())
}
