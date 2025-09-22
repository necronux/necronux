// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::model::UnifiedGrimoire;
use crate::ValidatedGrimoire;
use necronux_utils::trace_instrument;
use tracing::debug;

pub trait TryIntoUnifiedGrimoire {
    fn into_unified(self) -> UnifiedGrimoire;
}

impl TryIntoUnifiedGrimoire for ValidatedGrimoire {
    #[trace_instrument(level = "debug", skip(self))]
    fn into_unified(self) -> UnifiedGrimoire {
        debug!("Converting validated grimoire into unified grimoire...");

        let unified = match self {
            ValidatedGrimoire::CommonMetadata(g) => g.into(),
            #[cfg(feature = "grimoire_schema_v0")]
            ValidatedGrimoire::V0(g) => g.into(),
        };

        debug!("Successfully converted validated grimoire into unified grimoire");
        unified
    }
}
