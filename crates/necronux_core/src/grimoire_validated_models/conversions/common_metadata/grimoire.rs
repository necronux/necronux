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
        Ok(Self {
            schema_version: s.schema_version,
        })
    }
}
