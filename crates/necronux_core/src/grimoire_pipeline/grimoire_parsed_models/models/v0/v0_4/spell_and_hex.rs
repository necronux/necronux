// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::ParsedTool;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedSpell {
    // Option as RFB (See lib.rs)
    pub name: Option<String>,

    pub description: Option<String>,

    pub requires_confirmation: Option<bool>,

    pub keywords: Option<Vec<String>>,

    // Option as RFB (See lib.rs)
    pub invocations: Option<ParsedSpellInvocations>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedSpellInvocations {
    // Option as RFB (See lib.rs)
    pub cast: Option<ParsedInvocation>,

    // Option as RFB (See lib.rs)
    pub affirm: Option<ParsedInvocation>,

    // Option as RFB (See lib.rs)
    pub dispel: Option<ParsedInvocation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedHex {
    // Option as RFB (See lib.rs)
    pub name: Option<String>,

    pub description: Option<String>,

    pub requires_confirmation: Option<bool>,

    pub keywords: Option<Vec<String>>,

    // Option as RFB (See lib.rs)
    pub invocations: Option<ParsedHexInvocations>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedHexInvocations {
    // Option as RFB (See lib.rs)
    pub lay: Option<ParsedInvocation>,

    // Option as RFB (See lib.rs)
    pub discern: Option<ParsedInvocation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedInvocation {
    pub prefix_args: Option<String>,

    // Option as RFB (See lib.rs)
    pub execution_command: Option<String>,

    // Option as RFB (See lib.rs)
    pub instrument_path: Option<String>,

    // Option as RFB (See lib.rs)
    pub tool: Option<Vec<ParsedTool>>,
}
