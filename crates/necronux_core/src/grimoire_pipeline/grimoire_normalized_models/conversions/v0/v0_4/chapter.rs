// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError, grimoire_normalized_models::models::v0_4::NormalizedChapter,
    grimoire_parsed_models::models::v0_4::ParsedChapter, grimoire_pipeline::normalizers,
};
use std::collections::HashMap;

impl TryFrom<ParsedChapter> for NormalizedChapter {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedChapter) -> Result<Self, Self::Error> {
        let parent_object = "Chapter";
        Ok(Self {
            name: normalizers::ensure_req_field_is_not_missing(s.name, "name", parent_object)?,
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
