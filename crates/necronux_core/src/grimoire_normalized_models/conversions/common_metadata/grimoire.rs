// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError,
    grimoire_normalized_models::{
        models::common_metadata::{NormalizedGrimoire, NormalizedSchemaVersionInfo},
        normalizers,
    },
    grimoire_schemas::schemas::common_metadata::{ParsedGrimoire, ParsedSchemaVersionInfo},
};
use std::result as stdrt;

impl TryFrom<ParsedGrimoire> for NormalizedGrimoire {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedGrimoire) -> stdrt::Result<Self, Self::Error> {
        Ok(Self {
            schema_version_info: normalizers::ensure_top_level_req_field_is_not_missing(
                s.schema_version_info,
                "schemaVersionInfo",
            )?
            .try_into()?,
        })
    }
}

impl TryFrom<ParsedSchemaVersionInfo> for NormalizedSchemaVersionInfo {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedSchemaVersionInfo) -> stdrt::Result<Self, Self::Error> {
        let parent_object = "SchemaVersionInfo";
        Ok(Self {
            schema_version: normalizers::ensure_req_field_is_not_missing(
                s.schema_version,
                "schemaVersion",
                parent_object,
            )?,
        })
    }
}
