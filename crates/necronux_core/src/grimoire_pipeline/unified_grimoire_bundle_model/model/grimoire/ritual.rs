// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{UnifiedHex, UnifiedSpell};
use crate::unified_grimoire_bundle_model::SchemaField;

#[derive(Debug)]
pub struct UnifiedRitual {
    pub name: SchemaField<String>,
    pub description: SchemaField<Option<String>>,
    pub requires_confirmation: bool,
    pub keywords: SchemaField<Vec<String>>,
    pub steps: SchemaField<Vec<UnifiedRitualStep>>,
}

#[derive(Debug)]
pub enum UnifiedRitualStep {
    Cast(UnifiedRitualCastStep),
    Dispel(UnifiedRitualDispelStep),
    Lay(UnifiedRitualLayStep),
}

#[derive(Debug)]
pub struct UnifiedRitualCastStep {
    pub cast: SchemaField<UnifiedSpell>,
    pub requires_confirmation: bool,
    pub auto_affirm: bool,
}

#[derive(Debug)]
pub struct UnifiedRitualDispelStep {
    pub dispel: SchemaField<UnifiedSpell>,
    pub requires_confirmation: bool,
}

#[derive(Debug)]
pub struct UnifiedRitualLayStep {
    pub lay: SchemaField<UnifiedHex>,
    pub requires_confirmation: bool,
    pub auto_discern: bool,
}
