// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::missing;
use semver::Version;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedGrimoire {
    #[serde(default = "missing::schema_version_info")]
    pub schema_version_info: ParsedSchemaVersionInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedSchemaVersionInfo {
    // Option as SLRFB (See mod.rs)
    pub schema_version: Option<Version>,
}
