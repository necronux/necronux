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

    // Option as RFB (See mod.rs)
    pub magic_type: Option<String>,

    // Option as RFB (See mod.rs)
    pub name: Option<String>,

    pub description: Option<String>,

    // Option as RFB (See mod.rs)
    pub requires_confirmation: Option<bool>,

    pub keywords: Option<Vec<String>>,

    // Option as RFB (See mod.rs)
    pub cast_invocation: Option<ParsedInvocation>,

    // Option as RFB (See mod.rs)
    pub verify_invocation: Option<ParsedInvocation>,

    // Option as RFB (See mod.rs)
    pub dispel_invocation: Option<ParsedInvocation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedHex {
    #[serde(flatten)]
    pub grimoire_metadata: ParsedGrimoireMetadata,

    // Option as RFB (See mod.rs)
    pub magic_type: Option<String>,

    // Option as RFB (See mod.rs)
    pub name: Option<String>,

    pub description: Option<String>,

    // Option as RFB (See mod.rs)
    pub requires_confirmation: Option<bool>,

    pub keywords: Option<Vec<String>>,

    // Option as RFB (See mod.rs)
    pub cast_invocation: Option<ParsedInvocation>,

    // Option as RFB (See mod.rs)
    pub verify_invocation: Option<ParsedInvocation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedInvocation {
    pub prefix_args: Option<String>,

    // Option as RFB (See mod.rs)
    pub execution_command: Option<String>,

    // Option as RFB (See mod.rs)
    pub instrument_path: Option<String>,

    // Option as RFB (See mod.rs)
    pub tool: Option<Vec<ParsedTool>>,
}
