// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{ParsedGrimoireMetadata, ParsedHex, ParsedSpell};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedRitual {
    #[serde(flatten)]
    pub grimoire_metadata: ParsedGrimoireMetadata,

    // Option as RFB (See mod.rs)
    pub ritual_type: Option<String>,

    // Option as RFB (See mod.rs)
    pub name: Option<String>,

    pub description: Option<String>,

    // Option as RFB (See mod.rs)
    pub requires_confirmation: Option<bool>,

    pub keywords: Option<Vec<String>>,

    // Option as RFB (See mod.rs)
    pub steps: Option<Vec<ParsedRitualStep>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ParsedRitualStep {
    Cast(ParsedRitualCastStep),
    Dispel(ParsedRitualDispelStep),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedRitualCastStep {
    // Option as RFB (See mod.rs)
    pub cast: Option<ParsedSpellOrHex>,

    // Option as RFB (See mod.rs)
    pub requires_confirmation: Option<bool>,

    // Option as RFB (See mod.rs)
    pub auto_verify: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedRitualDispelStep {
    // Option as RFB (See mod.rs)
    pub dispel: Option<ParsedSpell>,

    // Option as RFB (See mod.rs)
    pub requires_confirmation: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum ParsedSpellOrHex {
    Spell(ParsedSpell),
    Hex(ParsedHex),
}

/* These are on hold
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedCompositeRitual {
    #[serde(flatten)]
    pub grimoire_metadata: ParsedGrimoireMetadata,

    pub ritual_type: String,

    pub name: String,

    pub description: Option<String>,

    pub requires_confirmation: bool,

    pub keywords: Option<Vec<String>>,

    pub steps: Vec<rpkl::Value>,
}
*/
