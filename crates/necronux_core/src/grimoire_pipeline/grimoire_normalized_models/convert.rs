// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::NormalizedGrimoire;
use crate::{error::NormalizeGrimoireErrorWithContext, grimoire_parsed_models::ParsedGrimoire};
use necronux_utils::trace_instrument;
use std::result as stdrt;
use tracing::debug;

impl ParsedGrimoire {
    #[trace_instrument(level = "debug", name = "parsed_grimoire_into_normalized", skip(self))]
    fn try_into_normalized(
        self,
    ) -> stdrt::Result<NormalizedGrimoire, NormalizeGrimoireErrorWithContext> {
        debug!("Converting parsed grimoire into normalized grimoire...");

        let normalized = match self {
            #[cfg(feature = "grimoire_schema_v0")]
            Self::V0_4(g_boxed) => {
                let g = *g_boxed;
                Ok(NormalizedGrimoire::V0_4(Box::new(g.try_into().map_err(
                    |e| NormalizeGrimoireErrorWithContext { source: e },
                )?)))
            }
        }?;

        debug!("Successfully converted parsed grimoire into normalized grimoire");
        Ok(normalized)
    }
}
