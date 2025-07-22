// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::UnifiedGrimoire;
use crate::{
    ParsedGrimoire,
    error::{EngineError, Result},
};

pub trait TryIntoUnifiedGrimoire {
    fn try_into_unified(self) -> Result<UnifiedGrimoire>;
}

impl TryIntoUnifiedGrimoire for ParsedGrimoire {
    fn try_into_unified(self) -> Result<UnifiedGrimoire> {
        fn try_into_unified_inner(g: ParsedGrimoire) -> Result<UnifiedGrimoire> {
            match g {
                ParsedGrimoire::MinimalMetadata(g) => Ok(UnifiedGrimoire::from(g)),
                #[cfg(feature = "grimoire_schema_v0")]
                ParsedGrimoire::V0(g) => Ok(UnifiedGrimoire::from(*g)),
            }
        }

        try_into_unified_inner(self).map_err(|e| EngineError::UnifyGrimoireError {
            source: Box::new(e),
        })
    }
}
