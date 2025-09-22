// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::NormalizedGrimoire;
use crate::{error::NormalizeGrimoireErrorWithContext, grimoire_schemas::ParsedGrimoire};
use necronux_utils::trace_instrument;
use std::result as stdrt;
use tracing::debug;

pub trait TryIntoNormalizedGrimoire {
    fn try_into_normalized(
        self,
    ) -> stdrt::Result<NormalizedGrimoire, NormalizeGrimoireErrorWithContext>;
}

impl TryIntoNormalizedGrimoire for ParsedGrimoire {
    #[trace_instrument(level = "debug", skip(self))]
    fn try_into_normalized(
        self,
    ) -> stdrt::Result<NormalizedGrimoire, NormalizeGrimoireErrorWithContext> {
        debug!("Converting parsed grimoire into normalized grimoire...");

        let normalized = match self {
            ParsedGrimoire::CommonMetadata(g) => Ok(NormalizedGrimoire::CommonMetadata(
                g.try_into()
                    .map_err(|e| NormalizeGrimoireErrorWithContext { source: e })?,
            )),
            #[cfg(feature = "grimoire_schema_v0")]
            ParsedGrimoire::V0(g) => {
                Ok(NormalizedGrimoire::V0(g.try_into().map_err(|e| {
                    NormalizeGrimoireErrorWithContext { source: e }
                })?))
            }
        }?;

        debug!("Successfully converted parsed grimoire into normalized grimoire");
        Ok(normalized)
    }
}
