// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::ParsedSchemaVersionInfo;

pub fn schema_version_info() -> ParsedSchemaVersionInfo {
    ParsedSchemaVersionInfo {
        schema_version: None,
    }
}
