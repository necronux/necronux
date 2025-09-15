// ==----------------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2024-2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==----------------------------------------------------------------== //

use crate::SubCmd;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "necronux", version, author, long_about = None, disable_help_flag = cfg!(feature = "experimental_pretty_cli"))]
pub struct CliOptions {
    #[cfg(feature = "experimental_pretty_cli")]
    #[arg(long, global = true, help = "Print help")]
    pub help: bool,

    #[command(subcommand)]
    pub subcommand: Option<SubCmd>,

    // -- COLOR AND UNICODE -- //
    #[arg(
        long,
        global = true,
        value_enum,
        help = "Set color output [env: CLICOLOR_FORCE=, CLICOLOR=, NO_COLOR=] [default: auto]"
    )]
    pub color: Option<ColorChoice>,

    #[arg(
        long,
        global = true,
        env = "NO_UNICODE",
        help = "Disable unicode characters in output"
    )]
    pub no_unicode: bool,

    // -- OUTPUT DETAILS AND FORMAT -- //
    #[arg(
        short,
        long,
        global = true,
        value_enum,
        default_value_t = CliProgressLevel::Minimal,
        help = "Set progress output level"
    )]
    pub progress: CliProgressLevel,

    #[arg(
        short,
        long,
        global = true,
        value_enum,
        default_value_t = CliOutputFormat::Human,
        help = "Set output format (stdout)"
    )]
    pub output_format: CliOutputFormat,

    // -- LOGGING (STDERR) -- //
    #[cfg(all(feature = "necronux_log", debug_assertions))]
    #[arg(
        long,
        global = true,
        value_enum,
        default_value_t = CliLogOutputFormat::FancyTree,
        help = "Set stderr log output format (debug-only)"
    )]
    pub log_output_format: CliLogOutputFormat,

    #[cfg(feature = "necronux_log")]
    #[arg(long, global = true, help = "Completely disable logging")]
    pub disable_logging: bool,

    #[cfg(feature = "necronux_log")]
    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity<clap_verbosity_flag::WarnLevel>,
}

#[derive(Debug, Copy, Clone, clap::ValueEnum)]
pub enum ColorChoice {
    #[value(
        help = "Always use colored output, regardless of terminal or environment [env: CLICOLOR_FORCE=]"
    )]
    Always,
    #[value(help = "Use colors only if output is a terminal [env: CLICOLOR=]")]
    Auto,
    #[value(
        help = "Never use colored output, regardless of terminal or environment [env: NO_COLOR=]"
    )]
    Never,
}

#[derive(Debug, Copy, Clone, clap::ValueEnum)]
pub enum CliProgressLevel {
    #[value(help = "Suppress all progress output")]
    None,
    #[value(help = "Show only high-level task progress output")]
    Minimal,
    #[value(help = "Show step-by-step detailed progress output")]
    Detailed,
}

#[derive(Debug, Copy, Clone, clap::ValueEnum)]
pub enum CliOutputFormat {
    #[value(help = "Human-readable output with colors and symbols")]
    Human,
    #[value(help = "Structured JSON output for machine readability")]
    Json,
}

#[cfg(all(feature = "necronux_log", debug_assertions))]
#[derive(Debug, Copy, Clone, clap::ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum CliLogOutputFormat {
    #[value(help = "Compact and readable single-line logs")]
    Fmt,
    #[value(help = "Same as `fmt` but with extended metadata like file and line")]
    FmtExtended,
    #[value(help = "Tree-style colorful logs for better hierarchy and readability")]
    FancyTree,
}

#[cfg(test)]
mod tests {
    use super::CliOptions;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        CliOptions::command().debug_assert();
    }
}
