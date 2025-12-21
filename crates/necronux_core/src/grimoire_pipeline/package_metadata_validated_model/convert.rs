// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::model::ValidatedPackageMetadata;
use crate::{
    error::ValidateGrimoireErrorWithContext,
    package_metadata_normalized_model::model::NormalizedPackageMetadata,
};
use necronux_utils::trace_instrument;
use std::result as stdrt;
use tracing::debug;

impl NormalizedPackageMetadata {
    #[trace_instrument(
        level = "debug",
        name = "normalized_package_metadata_into_validated",
        skip(self)
    )]
    pub fn try_into_validated(
        self,
    ) -> stdrt::Result<ValidatedPackageMetadata, ValidateGrimoireErrorWithContext> {
        debug!("Converting normalized grimoire package metadata into validated grimoire package metadata...");

        let normalized = self
            .try_into()
            .map_err(|e| ValidateGrimoireErrorWithContext { source: e })?;

        debug!("Successfully converted normalized grimoire package metadata into validated grimoire package metadata");
        Ok(normalized)
    }
}
