// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::UnifiedGrimoire;
use crate::ParsedGrimoire;
use necronux_utils::trace_instrument;
use tracing::debug;

pub trait TryIntoUnifiedGrimoire {
    fn into_unified(self) -> UnifiedGrimoire;
}

impl TryIntoUnifiedGrimoire for ParsedGrimoire {
    #[trace_instrument(level = "debug", skip(self))]
    fn into_unified(self) -> UnifiedGrimoire {
        debug!("Converting parsed grimoire into unified grimoire...");

        let unified = match self {
            ParsedGrimoire::MinimalMetadata(g) => UnifiedGrimoire::from(g),
            #[cfg(feature = "grimoire_schema_v0")]
            ParsedGrimoire::V0(g) => UnifiedGrimoire::from(*g),
        };

        debug!("Successfully converted parsed grimoire into unified grimoire");
        unified
    }
}
