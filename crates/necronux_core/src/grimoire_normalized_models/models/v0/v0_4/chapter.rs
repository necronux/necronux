// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{NormalizedGrimoireMetadata, NormalizedHex, NormalizedSpell};
use std::collections::HashMap;

#[derive(Debug)]
pub struct NormalizedChapter {
    pub grimoire_metadata: NormalizedGrimoireMetadata,
    pub name: String,
    pub description: Option<String>,
    pub spells: Option<HashMap<String, NormalizedSpell>>,
    pub hexes: Option<HashMap<String, NormalizedHex>>,
    pub requires_confirmation: bool,
}
