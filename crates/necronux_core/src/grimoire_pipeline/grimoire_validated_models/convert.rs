// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::ValidatedGrimoire;
use crate::{
    error::ValidateGrimoireErrorWithContext, grimoire_normalized_models::NormalizedGrimoire,
};
use necronux_utils::trace_instrument;
use std::result as stdrt;
use tracing::debug;

impl NormalizedGrimoire {
    #[trace_instrument(
        level = "debug",
        name = "normalized_grimoire_into_validated",
        skip(self)
    )]
    pub fn try_into_validated(
        self,
    ) -> stdrt::Result<ValidatedGrimoire, ValidateGrimoireErrorWithContext> {
        debug!("Converting normalized grimoire into validated grimoire...");

        let validated = match self {
            #[cfg(feature = "grimoire_schema_v0")]
            Self::V0_4(g_boxed) => {
                let g = *g_boxed;
                Ok(ValidatedGrimoire::V0_4(Box::new(g.try_into().map_err(
                    |e| ValidateGrimoireErrorWithContext { source: e },
                )?)))
            }
        }?;

        debug!("Successfully converted normalized grimoire into validated grimoire");
        Ok(validated)
    }
}
