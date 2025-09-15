// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::ParseGrimoireError;
use crate::grimoire_schemas::minimal_metadata as mm;
#[cfg(feature = "grimoire_schema_v0")]
use crate::grimoire_schemas::v0;
use std::result as stdrt;

pub trait GrimoireParser {
    fn name(&self) -> &str;

    fn parse(&self) -> stdrt::Result<ParsedGrimoire, ParseGrimoireError>;
}

#[derive(Debug)]
pub enum ParsedGrimoire {
    MinimalMetadata(mm::Grimoire),

    #[cfg(feature = "grimoire_schema_v0")]
    V0(Box<v0::Grimoire>),
}
