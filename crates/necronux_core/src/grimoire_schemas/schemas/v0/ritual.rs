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

    pub ritual_type: String,

    pub name: String,

    pub description: Option<String>,

    pub requires_confirmation: bool,

    pub keywords: Option<Vec<String>>,

    pub steps: Vec<ParsedRitualStep>,
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
    pub cast: ParsedSpellOrHex,

    pub requires_confirmation: bool,

    pub auto_verify: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedRitualDispelStep {
    pub dispel: ParsedSpell,

    pub requires_confirmation: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
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
