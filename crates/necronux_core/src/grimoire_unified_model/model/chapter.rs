// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{UnifiedGrimoireMetadata, UnifiedHex, UnifiedSpell};
use crate::grimoire_unified_model::SchemaField;
use std::collections::HashMap;

#[derive(Debug)]
pub struct UnifiedChapter {
    pub grimoire_metadata: SchemaField<UnifiedGrimoireMetadata>,
    pub name: SchemaField<String>,
    pub description: SchemaField<Option<String>>,
    pub spells: SchemaField<Option<HashMap<String, UnifiedSpell>>>,
    pub hexes: SchemaField<Option<HashMap<String, UnifiedHex>>>,
    pub requires_confirmation: SchemaField<bool>,
}
