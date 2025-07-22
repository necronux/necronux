// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    ParsedGrimoire,
    error::{EngineError, Result},
};

pub trait GrimoireValidator {
    fn validate(&self) -> Result<()>;

    fn validate_required_fields(&self) -> Result<()>;
}

impl GrimoireValidator for ParsedGrimoire {
    fn validate(&self) -> Result<()> {
        fn validate_inner(g: &ParsedGrimoire) -> Result<()> {
            match g {
                ParsedGrimoire::MinimalMetadata(g) => g.validate(),
                #[cfg(feature = "grimoire_schema_v0")]
                ParsedGrimoire::V0(g) => g.validate(),
            }
        }

        validate_inner(self).map_err(|e| EngineError::ValidateGrimoireError {
            source: Box::new(e),
        })
    }

    fn validate_required_fields(&self) -> Result<()> {
        match &self {
            ParsedGrimoire::MinimalMetadata(g) => g.validate_required_fields(),
            #[cfg(feature = "grimoire_schema_v0")]
            ParsedGrimoire::V0(g) => g.validate_required_fields(),
        }
    }
}
