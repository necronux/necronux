// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    common_metadata_normalized_model::model::{
        NormalizedCommonMetadata, NormalizedSchemaVersionInfo,
    },
    common_metadata_validated_model::model::{ValidatedCommonMetadata, ValidatedSchemaVersionInfo},
    error::ValidateGrimoireError,
};
use std::result as stdrt;

impl TryFrom<NormalizedCommonMetadata> for ValidatedCommonMetadata {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedCommonMetadata) -> stdrt::Result<Self, Self::Error> {
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
            schema_version: semver::Version::parse(&s.schema_version).map_err(|e| {
                ValidateGrimoireError::InvalidFieldValueSemverError {
                    field_name: "schemaVersion".to_string(),
                    value: s.schema_version.clone(),
                    parent_object_name: parent_object.to_string(),
                    source: e,
                }
            })?,
        })
    }
}
