// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{NormalizedGrimoireMetadata, NormalizedTool};

#[derive(Debug)]
pub struct NormalizedSpell {
    pub grimoire_metadata: NormalizedGrimoireMetadata,
    pub magic_type: String,
    pub name: String,
    pub description: Option<String>,
    pub requires_confirmation: bool,
    pub keywords: Option<Vec<String>>,
    pub cast_invocation: NormalizedInvocation,
    pub verify_invocation: NormalizedInvocation,
    pub dispel_invocation: NormalizedInvocation,
}

#[derive(Debug)]
pub struct NormalizedHex {
    pub grimoire_metadata: NormalizedGrimoireMetadata,
    pub magic_type: String,
    pub name: String,
    pub description: Option<String>,
    pub requires_confirmation: bool,
    pub keywords: Option<Vec<String>>,
    pub cast_invocation: NormalizedInvocation,
    pub verify_invocation: NormalizedInvocation,
}

#[derive(Debug)]
pub struct NormalizedInvocation {
    pub prefix_args: Option<String>,
    pub execution_command: String,
    pub instrument_path: String,
    pub tool: Vec<NormalizedTool>,
}
