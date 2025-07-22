// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{GrimoireMetadata, Hex, Spell};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Ritual {
    #[serde(flatten)]
    pub metadata: GrimoireMetadata,

    #[serde(rename = "ritualType")]
    pub ritual_type: String,

    pub name: String,

    pub description: Option<String>,

    #[serde(rename = "requiresConfirmation")]
    pub requires_confirmation: bool,

    pub keywords: Option<Vec<String>>,

    pub steps: Vec<RitualStep>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RitualStep {
    Cast(RitualCastStep),
    Dispel(RitualDispelStep),
}

#[derive(Debug, Deserialize)]
pub struct RitualCastStep {
    pub cast: SpellOrHex,

    #[serde(rename = "requiresConfirmation")]
    pub requires_confirmation: bool,

    #[serde(rename = "autoVerify")]
    pub auto_verify: bool,
}

#[derive(Debug, Deserialize)]
pub struct RitualDispelStep {
    pub dispel: Spell,

    #[serde(rename = "requiresConfirmation")]
    pub requires_confirmation: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SpellOrHex {
    Spell(Spell),
    Hex(Hex),
}

#[derive(Debug, Deserialize)]
pub struct CompositeRitual {
    #[serde(flatten)]
    pub metadata: GrimoireMetadata,

    #[serde(rename = "ritualType")]
    pub ritual_type: String,

    pub name: String,

    pub description: Option<String>,

    #[serde(rename = "requiresConfirmation")]
    pub requires_confirmation: bool,

    pub keywords: Option<Vec<String>>,

    pub steps: Vec<rpkl::Value>,
}
