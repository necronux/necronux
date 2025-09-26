// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::ValidateGrimoireError, grimoire_normalized_models::models::v0_4::NormalizedChapter,
    models::v0_4::ValidatedChapter, validators,
};
use std::collections::{HashMap, HashSet};

impl TryFrom<NormalizedChapter> for ValidatedChapter {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedChapter) -> Result<Self, Self::Error> {
        let parent_object = "Chapter";
        let spell_keys: HashSet<String> = s
            .spells
            .as_ref()
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default();
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            name: validators::ensure_str_is_not_empty(s.name, "name", parent_object)?,
            description: validators::ensure_some_str_is_not_empty(
                s.description,
                "description",
                parent_object,
            )?,
            spells: validators::ensure_some_map_is_not_empty(s.spells, "spells", parent_object)?
                .map(|s| {
                    s.into_iter()
                        .map(|(k, v)| {
                            let key =
                                validators::ensure_str_is_not_empty(k, "Spell Key", parent_object)?;
                            v.try_into().map(|nv| (key, nv))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()
                })
                .transpose()?,
            hexes: validators::ensure_some_map_is_not_empty(s.hexes, "hexes", parent_object)?
                .map(|h| {
                    h.into_iter()
                        .map(|(k, v)| {
                            let hex_key =
                                validators::ensure_str_is_not_empty(k, "Hex Key", parent_object)?;

                            if spell_keys.contains(&hex_key) {
                                return Err(ValidateGrimoireError::DuplicateKeyAcrossMaps {
                                    key: hex_key.clone(),
                                    reason: "A spell and a hex must not have the same key name"
                                        .to_string(),
                                });
                            }
                            v.try_into().map(|nv| (hex_key, nv))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()
                })
                .transpose()?,
            requires_confirmation: s.requires_confirmation,
        })
    }
}
