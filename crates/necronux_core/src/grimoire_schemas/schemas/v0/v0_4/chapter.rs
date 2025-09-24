// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{ParsedGrimoireMetadata, ParsedHex, ParsedSpell};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedChapter {
    #[serde(flatten)]
    pub grimoire_metadata: ParsedGrimoireMetadata,

    // Option as RFB (See mod.rs)
    pub name: Option<String>,

    pub description: Option<String>,

    pub spells: Option<HashMap<String, ParsedSpell>>,

    pub hexes: Option<HashMap<String, ParsedHex>>,

    // Option as RFB (See mod.rs)
    pub requires_confirmation: Option<bool>,
}
