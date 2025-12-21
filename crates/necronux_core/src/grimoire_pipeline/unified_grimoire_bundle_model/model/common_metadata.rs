// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use semver::Version;

#[derive(Debug)]
pub struct UnifiedCommonMetadata {
    pub schema_version_info: UnifiedSchemaVersionInfo,
}

#[derive(Debug)]
pub struct UnifiedSchemaVersionInfo {
    pub schema_version: Version,
}
