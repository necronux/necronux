// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{NormalizedGrimoireMetadata, NormalizedHex, NormalizedSpell};

#[derive(Debug)]
pub struct NormalizedRitual {
    pub grimoire_metadata: NormalizedGrimoireMetadata,
    pub ritual_type: String,
    pub name: String,
    pub description: Option<String>,
    pub requires_confirmation: bool,
    pub keywords: Option<Vec<String>>,
    pub steps: Vec<NormalizedRitualStep>,
}

#[derive(Debug)]
pub enum NormalizedRitualStep {
    Cast(NormalizedRitualCastStep),
    Dispel(NormalizedRitualDispelStep),
}

#[derive(Debug)]
pub struct NormalizedRitualCastStep {
    pub cast: NormalizedSpellOrHex,
    pub requires_confirmation: bool,
    pub auto_verify: bool,
}

#[derive(Debug)]
pub struct NormalizedRitualDispelStep {
    pub dispel: NormalizedSpell,
    pub requires_confirmation: bool,
}

#[derive(Debug)]
pub enum NormalizedSpellOrHex {
    Spell(NormalizedSpell),
    Hex(NormalizedHex),
}
