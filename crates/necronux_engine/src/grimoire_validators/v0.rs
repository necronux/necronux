// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::GrimoireValidator;
use crate::error::{EngineError, Result};

impl GrimoireValidator for crate::grimoire_schemas::v0::Grimoire {
    fn validate(&self) -> Result<()> {
        self.validate_required_fields()?;
        Ok(())
    }

    fn validate_required_fields(&self) -> Result<()> {
        if self.grimoire_metadata.grimoire_name.to_string().is_empty() {
            return Err(EngineError::MissingRequiredField {
                field_name: "grimoire_name".to_string(),
            });
        }

        Ok(())
    }
}
