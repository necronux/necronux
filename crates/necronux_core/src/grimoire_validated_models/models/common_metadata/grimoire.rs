// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use semver::Version;

#[derive(Debug)]
pub struct ValidatedGrimoire {
    pub schema_version_info: ValidatedSchemaVersionInfo,
}

#[derive(Debug)]
pub struct ValidatedSchemaVersionInfo {
    pub schema_version: Version,
}
