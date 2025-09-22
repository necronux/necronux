// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{ValidatedGrimoireMetadata, ValidatedHex, ValidatedSpell};

#[derive(Debug)]
pub struct ValidatedRitual {
    pub grimoire_metadata: ValidatedGrimoireMetadata,
    pub ritual_type: String,
    pub name: String,
    pub description: Option<String>,
    pub requires_confirmation: bool,
    pub keywords: Option<Vec<String>>,
    pub steps: Vec<ValidatedRitualStep>,
}

#[derive(Debug)]
pub enum ValidatedRitualStep {
    Cast(ValidatedRitualCastStep),
    Dispel(ValidatedRitualDispelStep),
}

#[derive(Debug)]
pub struct ValidatedRitualCastStep {
    pub cast: ValidatedSpellOrHex,
    pub requires_confirmation: bool,
    pub auto_verify: bool,
}

#[derive(Debug)]
pub struct ValidatedRitualDispelStep {
    pub dispel: ValidatedSpell,
    pub requires_confirmation: bool,
}

#[derive(Debug)]
pub enum ValidatedSpellOrHex {
    Spell(ValidatedSpell),
    Hex(ValidatedHex),
}
