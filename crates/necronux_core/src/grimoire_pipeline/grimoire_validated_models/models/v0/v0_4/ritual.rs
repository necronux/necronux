// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{ValidatedHex, ValidatedSpell};

#[derive(Debug)]
pub struct ValidatedRitual {
    pub name: String,
    pub description: Option<String>,
    pub requires_confirmation: Option<bool>,
    pub keywords: Option<Vec<String>>,
    pub steps: Vec<ValidatedRitualStep>,
}

#[derive(Debug)]
pub enum ValidatedRitualStep {
    Cast(ValidatedRitualCastStep),
    Dispel(ValidatedRitualDispelStep),
    Lay(ValidatedRitualLayStep),
}

#[derive(Debug)]
pub struct ValidatedRitualCastStep {
    pub cast: ValidatedSpell,
    pub requires_confirmation: Option<bool>,
    pub auto_affirm: Option<bool>,
}
#[derive(Debug)]
pub struct ValidatedRitualDispelStep {
    pub dispel: ValidatedSpell,
    pub requires_confirmation: Option<bool>,
}

#[derive(Debug)]
pub struct ValidatedRitualLayStep {
    pub lay: ValidatedHex,
    pub requires_confirmation: Option<bool>,
    pub auto_discern: Option<bool>,
}
