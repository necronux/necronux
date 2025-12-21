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
    Affirm(AffirmSubCmd),

    #[command(about = "Dispel")]
    Dispel(DispelSubCmd),

    #[command(about = "Lay")]
    Lay(LaySubCmd),

    #[command(about = "Discern")]
    Discern(DiscernSubCmd),

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
    #[arg(help = "Id of the spell to cast")]
    pub spell_id: String,
}

#[derive(Args, Debug, Clone)]
pub struct AffirmSubCmd {
    #[arg(help = "Id of the spell to affirm")]
    pub spell_id: String,
}

#[derive(Args, Debug, Clone)]
pub struct DispelSubCmd {
    #[arg(help = "Id of the spell to dispel")]
    pub spell_id: String,
}

#[derive(Args, Debug, Clone)]
pub struct LaySubCmd {
    #[arg(help = "Id of the hex to lay")]
    pub hex_id: String,
}

#[derive(Args, Debug, Clone)]
pub struct DiscernSubCmd {
    #[arg(help = "Id of the hex to discern")]
    pub hex_id: String,
}

#[derive(Args, Debug, Clone)]
pub struct PerformSubCmd {
    #[arg(help = "Id of the ritual to perform")]
    pub ritual_id: String,
}

#[derive(Args, Debug, Clone)]
pub struct InfoSubCmd {
    #[arg(help = "Id of the spell or hex or ritual to show information about")]
    pub id: String,
}
