// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::ValidateGrimoireError,
    grimoire_normalized_models::models::common_metadata::{
        NormalizedGrimoire, NormalizedSchemaVersionInfo,
    },
    models::common_metadata::{ValidatedGrimoire, ValidatedSchemaVersionInfo},
    validators,
};
use std::result as stdrt;

impl TryFrom<NormalizedGrimoire> for ValidatedGrimoire {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedGrimoire) -> stdrt::Result<Self, Self::Error> {
        Ok(Self {
            schema_version_info: s.schema_version_info.try_into()?,
        })
    }
}

impl TryFrom<NormalizedSchemaVersionInfo> for ValidatedSchemaVersionInfo {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedSchemaVersionInfo) -> stdrt::Result<Self, Self::Error> {
        let parent_object = "SchemaVersionInfo";
        Ok(Self {
            schema_version: {
                let ver = validators::ensure_str_is_not_empty(
                    s.schema_version.clone(),
                    "schemaVersion",
                    &parent_object,
                )?;
                semver::Version::parse(&ver).map_err(|e| {
                    ValidateGrimoireError::InvalidFieldValueSemverError {
                        field_name: "schemaVersion".to_string(),
                        value: s.schema_version.clone(),
                        parent_object_name: parent_object.to_string(),
                        source: e,
                    }
                })?
            },
        })
    }
}
