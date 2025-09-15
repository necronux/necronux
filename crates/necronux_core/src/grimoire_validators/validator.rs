// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    ParsedGrimoire,
    error::{CoreError, ValidateGrimoireError},
};
use std::result as stdrt;

pub trait GrimoireValidator {
    fn validate(&self) -> stdrt::Result<(), CoreError>;

    fn validate_required_fields(&self) -> stdrt::Result<(), ValidateGrimoireError>;
}

impl GrimoireValidator for ParsedGrimoire {
    fn validate(&self) -> stdrt::Result<(), CoreError> {
        match self {
            ParsedGrimoire::MinimalMetadata(g) => g.validate(),
            #[cfg(feature = "grimoire_schema_v0")]
            ParsedGrimoire::V0(g) => g.validate(),
        }
    }

    fn validate_required_fields(&self) -> stdrt::Result<(), ValidateGrimoireError> {
        match &self {
            ParsedGrimoire::MinimalMetadata(g) => g.validate_required_fields(),
            #[cfg(feature = "grimoire_schema_v0")]
            ParsedGrimoire::V0(g) => g.validate_required_fields(),
        }
    }
}
