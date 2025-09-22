// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{error::ParseGrimoireErrorWithContext, grimoire_schemas::ParsedGrimoire};
use std::result as stdrt;

pub trait GrimoireParser {
    fn name(&self) -> &str;

    fn parse(&self) -> stdrt::Result<ParsedGrimoire, ParseGrimoireErrorWithContext>;
}
