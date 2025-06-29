// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::Chapter;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct GrimoirePkl {
    pub grimoire: Grimoire,
}

#[derive(Deserialize)]
pub struct Grimoire {
    pub name: String,
    pub description: String,
    pub authors: String,
    pub chapters: HashMap<String, Chapter>,
}
