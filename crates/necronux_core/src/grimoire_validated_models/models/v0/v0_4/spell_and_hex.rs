// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{ValidatedGrimoireMetadata, ValidatedTool};

#[derive(Debug)]
pub struct ValidatedSpell {
    pub grimoire_metadata: ValidatedGrimoireMetadata,
    pub magic_type: String,
    pub name: String,
    pub description: Option<String>,
    pub requires_confirmation: bool,
    pub keywords: Option<Vec<String>>,
    pub cast_invocation: ValidatedInvocation,
    pub verify_invocation: ValidatedInvocation,
    pub dispel_invocation: ValidatedInvocation,
}

#[derive(Debug)]
pub struct ValidatedHex {
    pub grimoire_metadata: ValidatedGrimoireMetadata,
    pub magic_type: String,
    pub name: String,
    pub description: Option<String>,
    pub requires_confirmation: bool,
    pub keywords: Option<Vec<String>>,
    pub cast_invocation: ValidatedInvocation,
    pub verify_invocation: ValidatedInvocation,
}

#[derive(Debug)]
pub struct ValidatedInvocation {
    pub prefix_args: Option<String>,
    pub execution_command: String,
    pub instrument_path: String,
    pub tool: Vec<ValidatedTool>,
}
