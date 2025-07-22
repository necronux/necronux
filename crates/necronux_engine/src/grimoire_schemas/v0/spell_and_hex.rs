// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{GrimoireMetadata, Tool};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Spell {
    #[serde(flatten)]
    pub metadata: GrimoireMetadata,

    #[serde(rename = "magicType")]
    pub magic_type: String,

    pub name: String,

    pub description: Option<String>,

    #[serde(rename = "requiresConfirmation")]
    pub requires_confirmation: bool,

    pub keywords: Option<Vec<String>>,

    #[serde(rename = "castInvocation")]
    pub cast_invocation: Invocation,

    #[serde(rename = "verifyInvocation")]
    pub verify_invocation: Invocation,

    #[serde(rename = "dispelInvocation")]
    pub dispel_invocation: Invocation,
}

#[derive(Debug, Deserialize)]
pub struct Hex {
    #[serde(flatten)]
    pub metadata: GrimoireMetadata,

    #[serde(rename = "magicType")]
    pub magic_type: String,

    pub name: String,

    pub description: Option<String>,

    #[serde(rename = "requiresConfirmation")]
    pub requires_confirmation: bool,

    pub keywords: Option<Vec<String>>,

    #[serde(rename = "castInvocation")]
    pub cast_invocation: Invocation,

    #[serde(rename = "verifyInvocation")]
    pub verify_invocation: Invocation,
}

#[derive(Debug, Deserialize)]
pub struct Invocation {
    #[serde(rename = "prefixArgs")]
    pub prefix_args: Option<String>,

    #[serde(rename = "executionCommand")]
    pub execution_command: String,

    #[serde(rename = "instrumentPath")]
    pub instrument_path: String,

    pub tool: Vec<Tool>,
}
