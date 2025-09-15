// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

#[cfg(all(feature = "necronux_log", debug_assertions))]
use crate::CliLogOutputFormat;
use crate::{CliOptions, CliOutputFormat, CliProgressLevel, ColorChoice, SubCmd};

pub struct AppConfig {
    #[cfg(feature = "experimental_pretty_cli")]
    pub help: bool,

    pub subcommand: Option<SubCmd>,

    #[cfg(feature = "necronux_log")]
    pub verbosity: clap_verbosity_flag::Verbosity<clap_verbosity_flag::WarnLevel>,

    pub color: ColorChoice,
    pub unicode: bool,

    pub progress_level: CliProgressLevel,
    pub output_format: CliOutputFormat,

    #[cfg(all(feature = "necronux_log", debug_assertions))]
    pub log_output_format: CliLogOutputFormat,
    #[cfg(feature = "necronux_log")]
    pub logging: bool,
}

impl From<&CliOptions> for AppConfig {
    fn from(cli: &CliOptions) -> Self {
        AppConfig {
            #[cfg(feature = "experimental_pretty_cli")]
            help: cli.help,

            subcommand: cli.subcommand.clone(),

            #[cfg(feature = "necronux_log")]
            verbosity: cli.verbosity,

            color: determine_color_choice(cli.color),
            unicode: !cli.no_unicode,

            progress_level: cli.progress,
            output_format: cli.output_format,

            #[cfg(all(feature = "necronux_log", debug_assertions))]
            log_output_format: cli.log_output_format,
            #[cfg(feature = "necronux_log")]
            logging: !cli.disable_logging,
        }
    }
}

pub fn determine_color_choice(cli: Option<ColorChoice>) -> ColorChoice {
    if let Some(cli_color) = cli {
        // Override all if cli argument is present
        cli_color
    } else if std::env::var_os("NO_COLOR").is_some() {
        ColorChoice::Never
    } else if std::env::var_os("CLICOLOR_FORCE").is_some() {
        ColorChoice::Always
    } else if std::env::var_os("CLICOLOR").is_some() {
        ColorChoice::Auto
    } else {
        // If nothing present, default to this
        ColorChoice::Auto
    }
}
