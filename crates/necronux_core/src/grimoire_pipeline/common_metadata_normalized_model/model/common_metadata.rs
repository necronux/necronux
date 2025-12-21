// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

#[derive(Debug)]
pub struct NormalizedCommonMetadata {
    pub schema_version_info: NormalizedSchemaVersionInfo,
}

#[derive(Debug)]
pub struct NormalizedSchemaVersionInfo {
    pub schema_version: String,
}
