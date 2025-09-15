// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::CliOptions;
use clap::Parser;

impl CliOptions {
    pub fn parse() -> Self {
        match Self::try_parse() {
            Ok(parsed_cli) => parsed_cli,
            Err(err) => err.exit(),
        }
    }
}
