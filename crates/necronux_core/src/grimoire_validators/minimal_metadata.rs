// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::GrimoireValidator;
use crate::error::{CoreError, ValidateGrimoireError};
use necronux_utils::trace_instrument;
use std::result as stdrt;
use tracing::debug;

impl GrimoireValidator for crate::grimoire_schemas::minimal_metadata::Grimoire {
    #[trace_instrument(level = "debug", skip(self))]
    fn validate(&self) -> stdrt::Result<(), CoreError> {
        debug!("Validating grimoire...");

        self.validate_required_fields()
            .map_err(|e| CoreError::ValidateGrimoireError { source: e })?;

        debug!("Successfully validated grimoire");
        Ok(())
    }

    fn validate_required_fields(&self) -> stdrt::Result<(), ValidateGrimoireError> {
        if self.std_schema_version.to_string().is_empty() {
            return Err(ValidateGrimoireError::MissingRequiredFieldError {
                field_name: "std_schema_version".to_string(),
            });
        }
        Ok(())
    }
}
