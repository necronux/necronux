// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::ValidatedTool;

#[derive(Debug)]
pub struct ValidatedSpell {
    pub name: String,
    pub description: Option<String>,
    pub requires_confirmation: Option<bool>,
    pub keywords: Option<Vec<String>>,
    pub invocations: ValidatedSpellInvocations,
}

#[derive(Debug)]
pub struct ValidatedSpellInvocations {
    pub cast: ValidatedInvocation,
    pub affirm: ValidatedInvocation,
    pub dispel: ValidatedInvocation,
}

#[derive(Debug)]
pub struct ValidatedHex {
    pub name: String,
    pub description: Option<String>,
    pub requires_confirmation: Option<bool>,
    pub keywords: Option<Vec<String>>,
    pub invocations: ValidatedHexInvocations,
}

#[derive(Debug)]
pub struct ValidatedHexInvocations {
    pub lay: ValidatedInvocation,
    pub discern: ValidatedInvocation,
}

#[derive(Debug)]
pub struct ValidatedInvocation {
    pub prefix_args: Option<String>,
    pub execution_command: String,
    pub instrument_path: String,
    pub tool: Vec<ValidatedTool>,
}
