// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{ParsedHex, ParsedSpell};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedRitual {
    // Option as RFB (See lib.rs)
    pub name: Option<String>,

    pub description: Option<String>,

    pub requires_confirmation: Option<bool>,

    pub keywords: Option<Vec<String>>,

    // Option as RFB (See lib.rs)
    pub steps: Option<Vec<ParsedRitualStep>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ParsedRitualStep {
    Cast(ParsedRitualCastStep),
    Dispel(ParsedRitualDispelStep),
    Lay(ParsedRitualLayStep),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedRitualCastStep {
    // Option as RFB (See lib.rs)
    pub cast: Option<ParsedSpell>,

    pub requires_confirmation: Option<bool>,

    pub auto_affirm: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedRitualDispelStep {
    // Option as RFB (See lib.rs)
    pub dispel: Option<ParsedSpell>,

    pub requires_confirmation: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedRitualLayStep {
    // Option as RFB (See lib.rs)
    pub lay: Option<ParsedHex>,

    pub requires_confirmation: Option<bool>,

    pub auto_discern: Option<bool>,
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
