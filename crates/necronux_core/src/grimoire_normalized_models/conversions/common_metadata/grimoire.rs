// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError,
    grimoire_normalized_models::models::common_metadata::{
        NormalizedGrimoire, NormalizedSchemaVersionInfo,
    },
    grimoire_schemas::schemas::common_metadata::{ParsedGrimoire, ParsedSchemaVersionInfo},
};
use semver::Version;
use std::result as stdrt;

impl TryFrom<ParsedGrimoire> for NormalizedGrimoire {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedGrimoire) -> stdrt::Result<Self, Self::Error> {
        Ok(Self {
            schema_version_info: s
                .schema_version_info
                .ok_or_else(|| NormalizeGrimoireError::MissingTopLevelRequiredField {
                    field_name: "schemaVersionInfo".to_string(),
                })?
                .try_into()?,
        })
    }
}

impl TryFrom<ParsedSchemaVersionInfo> for NormalizedSchemaVersionInfo {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedSchemaVersionInfo) -> stdrt::Result<Self, Self::Error> {
        Ok(Self {
            schema_version: Version::parse(&s.schema_version.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "schemaVersion".to_string(),
                    parent_object_name: "SchemaVersionInfo".to_string(),
                }
            })?)?,
        })
    }
}
