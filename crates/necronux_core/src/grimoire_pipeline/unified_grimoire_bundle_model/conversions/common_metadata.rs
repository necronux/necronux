// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    common_metadata_validated_model::model::{ValidatedCommonMetadata, ValidatedSchemaVersionInfo},
    unified_grimoire_bundle_model::model::{UnifiedCommonMetadata, UnifiedSchemaVersionInfo},
};

impl From<ValidatedCommonMetadata> for UnifiedCommonMetadata {
    fn from(s: ValidatedCommonMetadata) -> Self {
        Self {
            schema_version_info: s.schema_version_info.into(),
        }
    }
}

impl From<ValidatedSchemaVersionInfo> for UnifiedSchemaVersionInfo {
    fn from(s: ValidatedSchemaVersionInfo) -> Self {
        Self {
            schema_version: s.schema_version,
        }
    }
}
