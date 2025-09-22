// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{ParsedGrimoireMetadata, ParsedTool};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedSpell {
    #[serde(flatten)]
    pub grimoire_metadata: ParsedGrimoireMetadata,

    pub magic_type: String,

    pub name: String,

    pub description: Option<String>,

    pub requires_confirmation: bool,

    pub keywords: Option<Vec<String>>,

    pub cast_invocation: ParsedInvocation,

    pub verify_invocation: ParsedInvocation,

    pub dispel_invocation: ParsedInvocation,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedHex {
    #[serde(flatten)]
    pub grimoire_metadata: ParsedGrimoireMetadata,

    pub magic_type: String,

    pub name: String,

    pub description: Option<String>,

    pub requires_confirmation: bool,

    pub keywords: Option<Vec<String>>,

    pub cast_invocation: ParsedInvocation,

    pub verify_invocation: ParsedInvocation,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedInvocation {
    pub prefix_args: Option<String>,

    pub execution_command: String,

    pub instrument_path: String,

    pub tool: Vec<ParsedTool>,
}
