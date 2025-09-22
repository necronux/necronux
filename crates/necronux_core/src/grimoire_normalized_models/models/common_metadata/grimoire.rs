// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use semver::Version;

#[derive(Debug)]
pub struct NormalizedGrimoire {
    pub schema_version_info: NormalizedSchemaVersionInfo,
}

#[derive(Debug)]
pub struct NormalizedSchemaVersionInfo {
    pub schema_version: Version,
}
