// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedGrimoire {
    // Option as RFB (See mod.rs)
    pub schema_version_info: Option<ParsedSchemaVersionInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedSchemaVersionInfo {
    // Option as RFB (See mod.rs)
    pub schema_version: Option<String>,
}
