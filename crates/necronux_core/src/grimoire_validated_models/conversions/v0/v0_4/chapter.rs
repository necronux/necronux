// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::ValidateGrimoireError, grimoire_normalized_models::models::v0_4::NormalizedChapter,
    models::v0_4::ValidatedChapter,
};
use std::collections::HashMap;

impl TryFrom<NormalizedChapter> for ValidatedChapter {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedChapter) -> Result<Self, Self::Error> {
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            name: s.name,
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
            requires_confirmation: s.requires_confirmation,
        })
    }
}
