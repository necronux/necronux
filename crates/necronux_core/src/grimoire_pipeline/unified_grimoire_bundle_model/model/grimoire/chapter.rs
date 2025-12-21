// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{UnifiedHex, UnifiedSpell};
use crate::unified_grimoire_bundle_model::SchemaField;
use std::collections::HashMap;

#[derive(Debug)]
pub struct UnifiedChapter {
    pub name: SchemaField<String>,
    pub description: SchemaField<Option<String>>,
    pub spells: SchemaField<HashMap<String, UnifiedSpell>>,
    pub hexes: SchemaField<HashMap<String, UnifiedHex>>,
    pub requires_confirmation: bool,
}
