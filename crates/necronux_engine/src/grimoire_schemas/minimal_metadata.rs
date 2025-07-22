// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use semver::Version;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Grimoire {
    #[serde(rename = "stdSchemaVersion")]
    pub std_schema_version: Version,
}
