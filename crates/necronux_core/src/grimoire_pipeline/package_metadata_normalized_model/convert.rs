// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::model::NormalizedPackageMetadata;
use crate::{
    error::NormalizeGrimoireErrorWithContext,
    package_metadata_parsed_model::model::ParsedPackageMetadata,
};
use necronux_utils::trace_instrument;
use std::result as stdrt;
use tracing::debug;

impl ParsedPackageMetadata {
    #[trace_instrument(
        level = "debug",
        name = "parsed_package_metadata_into_normalized",
        skip(self)
    )]
    pub fn try_into_normalized(
        self,
    ) -> stdrt::Result<NormalizedPackageMetadata, NormalizeGrimoireErrorWithContext> {
        debug!("Converting parsed grimoire package metadata into normalized grimoire package metadata...");

        let normalized = self
            .try_into()
            .map_err(|e| NormalizeGrimoireErrorWithContext { source: e })?;

        debug!("Successfully converted parsed grimoire package metadata into normalized grimoire package metadata");
        Ok(normalized)
    }
}
