// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::Cli;
use anyhow::Context;
use clap::CommandFactory;
use tracing::error;

pub enum HelpType {
    RootHelp,
    RepoHelp,
}

pub struct HelpHandler {
    help: HelpType,
}

impl HelpHandler {
    pub fn new(help: HelpType) -> Self {
        Self { help }
    }

    pub fn handle(&self) -> ! {
        match self
            .display_help()
            .context("Failed to display help message")
        {
            Ok(_) => std::process::exit(0),
            Err(err) => {
                error!("{}", err);
                std::process::exit(1)
            }
        }
    }

    pub fn display_help(&self) -> anyhow::Result<()> {
        match &self.help {
            HelpType::RootHelp => {
                #[cfg(feature = "experimental_pretty_cli")]
                {
                    // No subcommand support yet. Need extra wiring to include subcommands in the message.
                    let mut printer = clap_help::Printer::new(Cli::command())
                        .with("title", necronux_core::utils::statics::TITLE)
                        .with("introduction", necronux_core::utils::statics::INTRO)
                        .with("options", clap_help::TEMPLATE_OPTIONS_MERGED_VALUE);
                    let skin = printer.skin_mut();
                    skin.headers[0].compound_style.set_fg(termimad::ansi(141));
                    skin.bold.set_fg(termimad::ansi(141));
                    skin.italic = termimad::CompoundStyle::with_fg(termimad::ansi(141));
                    printer.print_help();
                }

                #[cfg(not(feature = "experimental_pretty_cli"))]
                Cli::command()
                    .print_help()
                    .context("Failed to print 'root' help message")?;
            }

            HelpType::RepoHelp => {
                let mut cmd = Cli::command();
                cmd.find_subcommand_mut("repo")
                    .context("Subcommand 'repo' not found")?
                    .print_help()
                    .context("Failed to print 'repo' help message")?;
            }
        }
        Ok(())
    }
}
