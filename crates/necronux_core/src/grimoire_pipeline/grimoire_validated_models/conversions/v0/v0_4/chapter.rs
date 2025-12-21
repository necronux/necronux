// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::ValidateGrimoireError, grimoire_normalized_models::models::v0_4::NormalizedChapter,
    grimoire_pipeline::validators, grimoire_validated_models::models::v0_4::ValidatedChapter,
};
use std::collections::{HashMap, HashSet};

impl TryFrom<NormalizedChapter> for ValidatedChapter {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedChapter) -> Result<Self, Self::Error> {
        let parent_object = "Chapter";
        let spell_keys: HashSet<String> = s
            .spells
            .as_ref()
            .map(|s| s.keys().cloned().collect())
            .unwrap_or_default();
        Ok(Self {
            name: validators::ensure_str_is_not_empty(s.name, "name", parent_object)?,
            description: s
                .description
                .map(|d| validators::ensure_str_is_not_empty(d, "description", parent_object))
                .transpose()?,
            spells: s
                .spells
                .map(|s| {
                    s.into_iter()
                        .map(|(k, v)| {
                            let key = validators::ensure_str_is_not_empty(
                                k,
                                "Spell Id (Key)",
                                parent_object,
                            )?;
                            let value = v.try_into()?;
                            Ok((key, value))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()
                })
                .transpose()?,
            hexes: s
                .hexes
                .map(|h| {
                    h.into_iter()
                        .map(|(k, v)| {
                            let hex_key = validators::ensure_str_is_not_empty(
                                k,
                                "Hex Id (Key)",
                                parent_object,
                            )?;
                            if spell_keys.contains(&hex_key) {
                                return Err(ValidateGrimoireError::DuplicateKeyAcrossMaps {
                                    key: hex_key.clone(),
                                    reason: "A spell and a hex must not have the same id (key)"
                                        .to_string(),
                                });
                            }
                            let value = v.try_into()?;
                            Ok((hex_key, value))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()
                })
                .transpose()?,
            requires_confirmation: s.requires_confirmation,
        })
    }
}
