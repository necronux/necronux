use crate::controller::Cli;
use color_eyre::eyre::{eyre, Context, Result};
use config::Config;
use log::{debug, LevelFilter};
use std::{env, path::Path};

// The order of precedence is:
// Default < Necronux Bundle File < Environment Variable < CLI Verbosity Flag
pub fn load_and_merge_configs(default_log_level: &LevelFilter, cli: &Cli) -> Result<Config> {
    debug!("Loading configs from various sources...");

    let mut builder = Config::builder();

    // Load default configs.
    let log_level_str = match default_log_level {
        LevelFilter::Trace => "trace",
        LevelFilter::Debug => "debug",
        LevelFilter::Info => "info",
        LevelFilter::Warn => "warn",
        LevelFilter::Error => "error",
        LevelFilter::Off => "off",
    };
    builder = builder.set_default("logging.level", log_level_str)?;

    // Override configs from the necronux bundle file.
    let config_dir = env::var("XDG_CONFIG_HOME")
        .or_else(|_| {
            env::var("HOME").map(|home| {
                Path::new(&home)
                    .join(".config")
                    .to_string_lossy()
                    .to_string()
            })
        })
        .context("Neither $XDG_CONFIG_HOME nor $HOME are set")?;
    let bundle_file_path = Path::new(&config_dir)
        .join("necronux")
        .join("bundle.necronux");
    let bundle_file_path_str = bundle_file_path.to_str().ok_or_else(|| {
        eyre!(
            "The path contains invalid UTF-8: {}",
            bundle_file_path.display()
        )
    })?;
    let bundle = config::File::new(bundle_file_path_str, config::FileFormat::Toml);
    if Path::new(bundle_file_path_str).exists() {
        debug!("Loading configuration from file: {}", bundle_file_path_str);
    } else {
        debug!("Configuration file not found at: {}", bundle_file_path_str);
    }
    builder = builder.add_source(bundle.required(false));

    // Override configs from the environment variables.
    if let Ok(env_log_level) = env::var("NECRONUX_LOG_LEVEL") {
        debug!(
            "Overriding log level from environment variable: {}",
            env_log_level
        );
        builder = builder.set_override("logging.level", env_log_level)?;
    }

    // Override configs from the cli verbosity flag.
    if cli.verbose.is_present() {
        let cli_log_level = cli.verbose.log_level_filter();
        let cli_log_level_str = match cli_log_level {
            LevelFilter::Trace => "trace",
            LevelFilter::Debug => "debug",
            LevelFilter::Info => "info",
            LevelFilter::Warn => "warn",
            LevelFilter::Error => "error",
            LevelFilter::Off => "off",
        };
        debug!(
            "Overriding log level from cli verbosity flag: {}",
            cli_log_level_str
        );
        builder = builder.set_override("logging.level", cli_log_level_str)?;
    }

    let merged_config = builder.build()?;
    debug!("Configs from various sources successfully loaded.");

    Ok(merged_config)
}
