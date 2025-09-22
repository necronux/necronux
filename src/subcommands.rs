// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use clap::Args;
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum SubCmd {
    #[command(about = "Bind a grimoire from a path or URL")]
    Bind(BindSubCmd),

    #[command(about = "Unbind the currently bound grimoire")]
    Unbind(UnbindSubCmd),

    #[command(about = "Validate the currently bound grimoire")]
    Validate(ValidateSubCmd),

    #[command(about = "Show Necronux status")]
    Status(StatusSubCmd),

    #[command(about = "Cast")]
    Cast(CastSubCmd),

    #[command(about = "Verify")]
    Verify(VerifySubCmd),

    #[command(about = "Dispel")]
    Dispel(DispelSubCmd),

    #[command(about = "Perform")]
    Perform(PerformSubCmd),

    #[command(about = "Info")]
    Info(InfoSubCmd),
}

#[derive(Args, Debug, Clone)]
pub struct BindSubCmd {
    #[arg(help = "Path or URL to the grimoire package source (package zip)")]
    pub source: String,

    #[arg(
        long,
        help = "Force binding by replacing the currently bound grimoire, if one exists"
    )]
    pub force: bool,
}

#[derive(Args, Debug, Copy, Clone)]
pub struct UnbindSubCmd {
    #[arg(long, help = "Force unbinding the grimoire even if one does not exist")]
    pub force: bool,
}

#[derive(Args, Debug, Copy, Clone)]
pub struct ValidateSubCmd {}

#[derive(Args, Debug, Copy, Clone)]
pub struct StatusSubCmd {
    #[arg(long, short, help = "Show full Necronux status")]
    pub full: bool,
}

#[derive(Args, Debug, Clone)]
pub struct CastSubCmd {
    #[arg(help = "Name of the spell or hex to cast")]
    pub name: String,
}

#[derive(Args, Debug, Clone)]
pub struct VerifySubCmd {
    #[arg(help = "Name of the spell or hex to verify")]
    pub name: String,
}

#[derive(Args, Debug, Clone)]
pub struct DispelSubCmd {
    #[arg(help = "Name of the spell to dispel")]
    pub name: String,
}

#[derive(Args, Debug, Clone)]
pub struct PerformSubCmd {
    #[arg(help = "Name of the ritual to perform")]
    pub name: String,
}

#[derive(Args, Debug, Clone)]
pub struct InfoSubCmd {
    #[arg(help = "Name of the spell or hex or ritual to show information about")]
    pub name: String,
}
