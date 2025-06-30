// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum SubCmd {
    #[command()]
    Connect(ConnectSubCmd),

    #[command()]
    Validate(ValidateSubCmd),
}

pub use engine::*;
mod engine {
    use clap::Args;

    #[derive(Args, Debug)]
    pub struct ConnectSubCmd {}

    #[derive(Args, Debug)]
    pub struct ValidateSubCmd {}
}
