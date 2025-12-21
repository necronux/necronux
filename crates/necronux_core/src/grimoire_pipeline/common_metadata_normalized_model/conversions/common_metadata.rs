// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    common_metadata_normalized_model::model::{
        NormalizedCommonMetadata, NormalizedSchemaVersionInfo,
    },
    common_metadata_parsed_model::model::{ParsedCommonMetadata, ParsedSchemaVersionInfo},
    error::NormalizeGrimoireError,
    grimoire_pipeline::normalizers,
};
use std::result as stdrt;

impl TryFrom<ParsedCommonMetadata> for NormalizedCommonMetadata {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedCommonMetadata) -> stdrt::Result<Self, Self::Error> {
        let parent_object = "necronux.grimoire file";
        Ok(Self {
            schema_version_info: normalizers::ensure_req_field_is_not_missing(
                s.schema_version_info,
                "schemaVersionInfo",
                parent_object,
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
