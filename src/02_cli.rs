// ==----------------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2024-2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==----------------------------------------------------------------== //

use clap::Parser;

#[cfg(not(feature = "experimental_pretty_cli"))]
#[derive(Parser, Debug)]
#[command(name = "necronux", version, author, long_about = None)]
pub struct Cli {
    #[cfg(feature = "cli_arg_log")]
    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity<clap_verbosity_flag::WarnLevel>,

    #[command(flatten)]
    pub shared: CliShared,
}

#[cfg(feature = "experimental_pretty_cli")]
#[derive(Parser, Debug)]
#[command(name = "necronux", version, author, long_about = None, disable_help_flag = true)]
pub struct Cli {
    #[cfg(feature = "cli_arg_log")]
    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity<clap_verbosity_flag::WarnLevel>,

    #[arg(long, global = true, help = "Print help")]
    pub help: bool,

    #[command(flatten)]
    pub shared: CliShared,
}

#[derive(Parser, Debug)]
pub struct CliShared {
    #[command(subcommand)]
    pub subcommand: Option<crate::SubCmd>,

    #[arg(
        short,
        long,
        global = true,
        help = "Show step-by-step progress details"
    )]
    pub progress: bool,

    #[arg(long, global = true, env = "NO_COLOR", help = "Disable colored output")]
    pub no_color: bool,

    #[cfg(debug_assertions)]
    #[arg(long, global = true, help = "Show extended log metadata (debug-only)")]
    pub log_extended: bool,
}

impl Cli {
    pub fn init() -> Self {
        match Self::try_parse() {
            Ok(cli) => cli,
            Err(err) => err.exit(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
