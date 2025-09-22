// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{UnifiedGrimoireMetadata, UnifiedHex, UnifiedSpell};
use crate::grimoire_unified_model::SchemaField;

#[derive(Debug)]
pub struct UnifiedRitual {
    pub grimoire_metadata: SchemaField<UnifiedGrimoireMetadata>,
    pub ritual_type: SchemaField<String>,
    pub name: SchemaField<String>,
    pub description: SchemaField<Option<String>>,
    pub requires_confirmation: SchemaField<bool>,
    pub keywords: SchemaField<Option<Vec<String>>>,
    pub steps: SchemaField<Vec<UnifiedRitualStep>>,
}

#[derive(Debug)]
pub enum UnifiedRitualStep {
    Cast(UnifiedRitualCastStep),
    Dispel(UnifiedRitualDispelStep),
}

#[derive(Debug)]
pub struct UnifiedRitualCastStep {
    pub cast: SchemaField<UnifiedSpellOrHex>,
    pub requires_confirmation: SchemaField<bool>,
    pub auto_verify: SchemaField<bool>,
}

#[derive(Debug)]
pub struct UnifiedRitualDispelStep {
    pub dispel: SchemaField<UnifiedSpell>,
    pub requires_confirmation: SchemaField<bool>,
}

#[derive(Debug)]
pub enum UnifiedSpellOrHex {
    Spell(UnifiedSpell),
    Hex(UnifiedHex),
}
