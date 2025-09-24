// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError, grimoire_normalized_models::models::v0_4::NormalizedChapter,
    grimoire_schemas::schemas::v0_4::ParsedChapter,
};
use std::collections::HashMap;

impl TryFrom<ParsedChapter> for NormalizedChapter {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedChapter) -> Result<Self, Self::Error> {
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            name: s
                .name
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "name".to_string(),
                    parent_object_name: "Chapter".to_string(),
                })?,
            description: s.description,
            spells: s
                .spells
                .map(|s| {
                    s.into_iter()
                        .map(|(k, v)| v.try_into().map(|nv| (k, nv)))
                        .collect::<Result<HashMap<_, _>, _>>()
                })
                .transpose()?,
            hexes: s
                .hexes
                .map(|h| {
                    h.into_iter()
                        .map(|(k, v)| v.try_into().map(|nv| (k, nv)))
                        .collect::<Result<HashMap<_, _>, _>>()
                })
                .transpose()?,
            requires_confirmation: s.requires_confirmation.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "requiresConfirmation".to_string(),
                    parent_object_name: "Chapter".to_string(),
                }
            })?,
        })
    }
}
