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
    #[command(subcommand)]
    pub subcommand: Option<crate::SubCmd>,

    #[cfg(feature = "cli_arg_log")]
    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity<clap_verbosity_flag::WarnLevel>,
}

#[cfg(feature = "experimental_pretty_cli")]
#[derive(Parser, Debug)]
#[command(name = "necronux", version, author, long_about = None, disable_help_flag = true)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<crate::SubCmd>,

    #[cfg(feature = "cli_arg_log")]
    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity<clap_verbosity_flag::WarnLevel>,

    /// Print help
    #[arg(long, global = true)]
    pub help: bool,
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
