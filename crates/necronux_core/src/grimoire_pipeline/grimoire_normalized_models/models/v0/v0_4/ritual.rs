// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{NormalizedHex, NormalizedSpell};

#[derive(Debug)]
pub struct NormalizedRitual {
    pub name: String,
    pub description: Option<String>,
    pub requires_confirmation: Option<bool>,
    pub keywords: Option<Vec<String>>,
    pub steps: Vec<NormalizedRitualStep>,
}

#[derive(Debug)]
pub enum NormalizedRitualStep {
    Cast(NormalizedRitualCastStep),
    Dispel(NormalizedRitualDispelStep),
    Lay(NormalizedRitualLayStep),
}

#[derive(Debug)]
pub struct NormalizedRitualCastStep {
    pub cast: NormalizedSpell,
    pub requires_confirmation: Option<bool>,
    pub auto_affirm: Option<bool>,
}

#[derive(Debug)]
pub struct NormalizedRitualDispelStep {
    pub dispel: NormalizedSpell,
    pub requires_confirmation: Option<bool>,
}

#[derive(Debug)]
pub struct NormalizedRitualLayStep {
    pub lay: NormalizedHex,
    pub requires_confirmation: Option<bool>,
    pub auto_discern: Option<bool>,
}
