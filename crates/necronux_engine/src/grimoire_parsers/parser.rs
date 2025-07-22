// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::Result;

pub trait GrimoireParser {
    fn name(&self) -> &str;

    fn parse(&self) -> Result<ParsedGrimoire>;
}

pub enum ParsedGrimoire {
    MinimalMetadata(crate::grimoire_schemas::minimal_metadata::Grimoire),

    #[cfg(feature = "grimoire_schema_v0")]
    V0(Box<crate::grimoire_schemas::v0::Grimoire>),
}
