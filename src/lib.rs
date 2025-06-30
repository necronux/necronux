// ==----------------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2024-2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==----------------------------------------------------------------== //

#[path = "02_cli.rs"]
mod cli;
#[path = "03_dispatch.rs"]
mod dispatch;
#[path = "01_run.rs"]
mod run;

mod handlers;
mod subcommands;

pub use cli::*;
pub use handlers::*;
pub use subcommands::*;
