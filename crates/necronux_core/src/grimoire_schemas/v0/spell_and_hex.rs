// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{GrimoireMetadata, Tool};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spell {
    #[serde(flatten)]
    pub metadata: GrimoireMetadata,

    pub magic_type: String,

    pub name: String,

    pub description: Option<String>,

    pub requires_confirmation: bool,

    pub keywords: Option<Vec<String>>,

    pub cast_invocation: Invocation,

    pub verify_invocation: Invocation,

    pub dispel_invocation: Invocation,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hex {
    #[serde(flatten)]
    pub metadata: GrimoireMetadata,

    pub magic_type: String,

    pub name: String,

    pub description: Option<String>,

    pub requires_confirmation: bool,

    pub keywords: Option<Vec<String>>,

    pub cast_invocation: Invocation,

    pub verify_invocation: Invocation,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invocation {
    pub prefix_args: Option<String>,

    pub execution_command: String,

    pub instrument_path: String,

    pub tool: Vec<Tool>,
}
