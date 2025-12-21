// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::model::ValidatedCommonMetadata;
use crate::{
    common_metadata_normalized_model::model::NormalizedCommonMetadata,
    error::ValidateGrimoireErrorWithContext,
};
use necronux_utils::trace_instrument;
use std::result as stdrt;
use tracing::debug;

impl NormalizedCommonMetadata {
    #[trace_instrument(
        level = "debug",
        name = "normalized_common_metadata_into_validated",
        skip(self)
    )]
    pub fn try_into_validated(
        self,
    ) -> stdrt::Result<ValidatedCommonMetadata, ValidateGrimoireErrorWithContext> {
        debug!("Converting normalized grimoire common metadata into validated grimoire common metadata...");

        let normalized = self
            .try_into()
            .map_err(|e| ValidateGrimoireErrorWithContext { source: e })?;

        debug!("Successfully converted normalized grimoire common metadata into validated grimoire common metadata");
        Ok(normalized)
    }
}
