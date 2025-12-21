// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::ValidateGrimoireError,
    grimoire_normalized_models::models::v0_4::{
        NormalizedCoreContents, NormalizedGrimoire, NormalizedGrimoireMetadata,
    },
    grimoire_pipeline::validators,
    grimoire_validated_models::models::v0_4::{
        ValidatedCoreContents, ValidatedGrimoire, ValidatedGrimoireMetadata,
    },
};
use std::{collections::HashMap, result as stdrt};
impl TryFrom<NormalizedGrimoire> for ValidatedGrimoire {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedGrimoire) -> stdrt::Result<Self, Self::Error> {
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.map(|m| m.try_into()).transpose()?,
            core_contents: s.core_contents.try_into()?,
        })
    }
}

impl TryFrom<NormalizedGrimoireMetadata> for ValidatedGrimoireMetadata {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedGrimoireMetadata) -> stdrt::Result<Self, Self::Error> {
        let parent_object = "GrimoireMetadata";
        Ok(Self {
            grimoire_keywords: s
                .grimoire_keywords
                .map(|k| {
                    k.into_iter()
                        .map(|kw| validators::ensure_str_is_not_empty(kw, "Keyword", parent_object))
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            // Always keep this field flexible
            grimoire_additional_metadata: s.grimoire_additional_metadata,
        })
    }
}

impl TryFrom<NormalizedCoreContents> for ValidatedCoreContents {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedCoreContents) -> stdrt::Result<Self, Self::Error> {
        let parent_object = "CoreContents";
        Ok(Self {
            chapters: s
                .chapters
                .map(|c| {
                    c.into_iter()
                        .map(|(k, v)| {
                            let key = validators::ensure_str_is_not_empty(
                                k,
                                "Chapter Id (Key)",
                                parent_object,
                            )?;
                            let value = v.try_into()?;
                            Ok((key, value))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()
                })
                .transpose()?,
            rituals: s
                .rituals
                .map(|r| {
                    r.into_iter()
                        .map(|(k, v)| {
                            let key = validators::ensure_str_is_not_empty(
                                k,
                                "Ritual Id (Key)",
                                parent_object,
                            )?;
                            let value = v.try_into()?;
                            Ok((key, value))
                        })
                        .collect::<Result<HashMap<_, _>, _>>()
                })
                .transpose()?,
            auto_perform_rituals: s.auto_perform_rituals,
            requires_confirmation: s.requires_confirmation,
        })
    }
}
