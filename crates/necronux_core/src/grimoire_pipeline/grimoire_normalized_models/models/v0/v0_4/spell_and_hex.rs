// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::NormalizedTool;

#[derive(Debug)]
pub struct NormalizedSpell {
    pub name: String,
    pub description: Option<String>,
    pub requires_confirmation: Option<bool>,
    pub keywords: Option<Vec<String>>,
    pub invocations: NormalizedSpellInvocations,
}

#[derive(Debug)]
pub struct NormalizedSpellInvocations {
    pub cast: NormalizedInvocation,
    pub affirm: NormalizedInvocation,
    pub dispel: NormalizedInvocation,
}

#[derive(Debug)]
pub struct NormalizedHex {
    pub name: String,
    pub description: Option<String>,
    pub requires_confirmation: Option<bool>,
    pub keywords: Option<Vec<String>>,
    pub invocations: NormalizedHexInvocations,
}

#[derive(Debug)]
pub struct NormalizedHexInvocations {
    pub lay: NormalizedInvocation,
    pub discern: NormalizedInvocation,
}

#[derive(Debug)]
pub struct NormalizedInvocation {
    pub prefix_args: Option<String>,
    pub execution_command: String,
    pub instrument_path: String,
    pub tool: Vec<NormalizedTool>,
}
