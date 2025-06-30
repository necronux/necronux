// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Chapter {
    pub name: String,
    pub description: String,
    pub spells: HashMap<String, Spell>,
    pub hexes: HashMap<String, Hex>,
}

#[derive(Deserialize)]
pub struct Spell {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct Hex {
    pub name: String,
    pub description: String,
}
