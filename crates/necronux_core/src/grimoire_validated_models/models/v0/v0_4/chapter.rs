// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{ValidatedGrimoireMetadata, ValidatedHex, ValidatedSpell};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ValidatedChapter {
    pub grimoire_metadata: ValidatedGrimoireMetadata,
    pub name: String,
    pub description: Option<String>,
    pub spells: Option<HashMap<String, ValidatedSpell>>,
    pub hexes: Option<HashMap<String, ValidatedHex>>,
    pub requires_confirmation: bool,
}
