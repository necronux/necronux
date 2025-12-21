// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::model::NormalizedCommonMetadata;
use crate::{
    common_metadata_parsed_model::model::ParsedCommonMetadata,
    error::NormalizeGrimoireErrorWithContext,
};
use necronux_utils::trace_instrument;
use std::result as stdrt;
use tracing::debug;

impl ParsedCommonMetadata {
    #[trace_instrument(
        level = "debug",
        name = "parsed_common_metadata_into_normalized",
        skip(self)
    )]
    pub fn try_into_normalized(
        self,
    ) -> stdrt::Result<NormalizedCommonMetadata, NormalizeGrimoireErrorWithContext> {
        debug!("Converting parsed grimoire common metadata into normalized grimoire common metadata...");

        let normalized = self
            .try_into()
            .map_err(|e| NormalizeGrimoireErrorWithContext { source: e })?;

        debug!("Successfully converted parsed grimoire common metadata into normalized grimoire common metadata");
        Ok(normalized)
    }
}
