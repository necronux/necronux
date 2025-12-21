// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError,
    grimoire_normalized_models::models::v0_4::{
        NormalizedCoreContents, NormalizedGrimoire, NormalizedGrimoireMetadata,
    },
    grimoire_parsed_models::models::v0_4::{
        ParsedCoreContents, ParsedGrimoire, ParsedGrimoireMetadata,
    },
    grimoire_pipeline::normalizers,
};
use std::{collections::HashMap, result as stdrt};

impl TryFrom<ParsedGrimoire> for NormalizedGrimoire {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedGrimoire) -> stdrt::Result<Self, Self::Error> {
        let parent_object = "necronux.grimoire file";
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.map(|m| m.try_into()).transpose()?,
            core_contents: normalizers::ensure_req_field_is_not_missing(
                s.core_contents,
                "coreContents",
                parent_object,
            )?
            .try_into()?,
        })
    }
}

impl TryFrom<ParsedGrimoireMetadata> for NormalizedGrimoireMetadata {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedGrimoireMetadata) -> stdrt::Result<Self, Self::Error> {
        Ok(Self {
            grimoire_keywords: s.grimoire_keywords,
            grimoire_additional_metadata: s.grimoire_additional_metadata,
        })
    }
}

impl TryFrom<ParsedCoreContents> for NormalizedCoreContents {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedCoreContents) -> stdrt::Result<Self, Self::Error> {
        Ok(Self {
            chapters: s
                .chapters
                .map(|c| {
                    c.into_iter()
                        .map(|(k, v)| v.try_into().map(|nv| (k, nv)))
                        .collect::<Result<HashMap<_, _>, _>>()
                })
                .transpose()?,
            rituals: s
                .rituals
                .map(|r| {
                    r.into_iter()
                        .map(|(k, v)| v.try_into().map(|nv| (k, nv)))
                        .collect::<Result<HashMap<_, _>, _>>()
                })
                .transpose()?,
            auto_perform_rituals: s.auto_perform_rituals,
            requires_confirmation: s.requires_confirmation,
        })
    }
}
