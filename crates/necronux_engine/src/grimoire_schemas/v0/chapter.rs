// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{GrimoireMetadata, Hex, Spell};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Chapter {
    #[serde(flatten)]
    pub metadata: GrimoireMetadata,

    pub name: String,

    pub description: Option<String>,

    pub spells: Option<HashMap<String, Spell>>,

    pub hexes: Option<HashMap<String, Hex>>,

    #[serde(rename = "requiresConfirmation")]
    pub requires_confirmation: bool,
}
