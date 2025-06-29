// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum SubCmd {
    #[cfg(feature = "experimental_engine")]
    #[command()]
    Init(InitSubCmd),

    #[cfg(feature = "experimental_auth")]
    #[command()]
    Auth(AuthSubCmd),

    #[command(arg_required_else_help = true)]
    Repo(RepoSubCmd),

    #[cfg(feature = "stdschema_v1")]
    #[command()]
    Validate(ValidateSubCmd),

    #[cfg(feature = "experimental_engine")]
    #[command()]
    Apply(ApplySubCmd),

    #[cfg(feature = "experimental_engine")]
    #[command()]
    Revert(RevertSubCmd),

    #[cfg(feature = "experimental_engine")]
    #[command()]
    Verify(VerifySubCmd),

    #[cfg(feature = "experimental_discovery")]
    #[command()]
    Search(SearchSubCmd),

    #[cfg(feature = "experimental_discovery")]
    #[command()]
    Catalog(CatalogSubCmd),

    #[cfg(feature = "experimental_discovery")]
    #[command()]
    Explore(ExploreSubCmd),

    #[cfg(feature = "experimental_discovery")]
    #[command()]
    Info(InfoSubCmd),

    #[cfg(feature = "experimental_engine")]
    #[command()]
    Status(StatusSubCmd),

    #[cfg(feature = "experimental_history")]
    #[command()]
    History(HistorySubCmd),

    #[cfg(feature = "experimental_engine")]
    #[command()]
    Clean(CleanSubCmd),
}

#[cfg(feature = "stdschema_v1")]
pub use engine::*;
#[cfg(feature = "stdschema_v1")]
mod engine {
    use clap::Args;

    #[cfg(feature = "experimental_engine")]
    #[derive(Args, Debug)]
    pub struct InitSubCmd {}

    #[cfg(feature = "stdschema_v1")]
    #[derive(Args, Debug)]
    pub struct ValidateSubCmd {}

    #[cfg(feature = "experimental_engine")]
    #[derive(Args, Debug)]
    pub struct ApplySubCmd {}

    #[cfg(feature = "experimental_engine")]
    #[derive(Args, Debug)]
    pub struct RevertSubCmd {}

    #[cfg(feature = "experimental_engine")]
    #[derive(Args, Debug)]
    pub struct VerifySubCmd {}

    #[cfg(feature = "experimental_engine")]
    #[derive(Args, Debug)]
    pub struct StatusSubCmd {}

    #[cfg(feature = "experimental_engine")]
    #[derive(Args, Debug)]
    pub struct CleanSubCmd {}
}

#[cfg(feature = "experimental_auth")]
pub use auth::*;
#[cfg(feature = "experimental_auth")]
mod auth {
    use clap::{Args, Subcommand};

    #[derive(Args, Debug)]
    pub struct AuthSubCmd {
        #[command(subcommand)]
        pub subcommand: Option<AuthSubSubCmd>,
    }

    #[derive(Subcommand, Debug)]
    pub enum AuthSubSubCmd {
        Add,
        Remove,
        List,
        Status,
    }
}

pub use vcs::*;
mod vcs {
    use clap::{Args, Subcommand};

    #[derive(Args, Debug)]
    pub struct RepoSubCmd {
        #[command(subcommand)]
        pub subcommand: Option<RepoSubSubCmd>,
    }

    #[derive(Subcommand, Debug)]
    pub enum RepoSubSubCmd {
        Connect(Connect),
        Reconnect,
        Disconnect,
        Switch(Switch),
        Update,
        Status,
    }

    #[derive(Debug, Args)]
    #[command(args_conflicts_with_subcommands = true)]
    pub struct Connect {
        pub url: String,

        #[arg(short = 'r', long, help = "Branch, tag, commit hash or bookmark")]
        pub revision: Option<String>,
    }

    #[derive(Debug, Args)]
    #[command(args_conflicts_with_subcommands = true)]
    pub struct Switch {
        pub url: String,

        #[arg(short = 'r', long, help = "Branch, tag, commit hash or bookmark")]
        pub revision: Option<String>,
    }
}

#[cfg(feature = "experimental_discovery")]
pub use discovery::*;
#[cfg(feature = "experimental_discovery")]
mod discovery {
    use clap::Args;

    #[derive(Args, Debug)]
    pub struct SearchSubCmd {}

    #[derive(Args, Debug)]
    pub struct CatalogSubCmd {}

    #[derive(Args, Debug)]
    pub struct ExploreSubCmd {}

    #[derive(Args, Debug)]
    pub struct InfoSubCmd {}
}

#[cfg(feature = "experimental_history")]
pub use history::*;
#[cfg(feature = "experimental_history")]
mod history {
    use clap::Args;
    #[derive(Args, Debug)]
    pub struct HistorySubCmd {}
}
