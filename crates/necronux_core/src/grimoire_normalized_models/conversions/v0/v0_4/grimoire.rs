// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError,
    grimoire_normalized_models::{
        models::v0_4::{NormalizedCoreContents, NormalizedGrimoire, NormalizedGrimoireMetadata},
        normalizers,
    },
    grimoire_schemas::schemas::v0_4::{ParsedCoreContents, ParsedGrimoire, ParsedGrimoireMetadata},
};
use std::{collections::HashMap, result as stdrt};

impl TryFrom<ParsedGrimoire> for NormalizedGrimoire {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedGrimoire) -> stdrt::Result<Self, Self::Error> {
        Ok(Self {
            common_metadata: s.common_metadata.try_into()?,
            grimoire_metadata: normalizers::ensure_top_level_req_field_is_not_missing(
                s.grimoire_metadata,
                "grimoireMetadata",
            )?
            .try_into()?,
            core_contents: normalizers::ensure_top_level_req_field_is_not_missing(
                s.core_contents,
                "coreContents",
            )?
            .try_into()?,
        })
    }
}

impl TryFrom<ParsedGrimoireMetadata> for NormalizedGrimoireMetadata {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedGrimoireMetadata) -> stdrt::Result<Self, Self::Error> {
        let parent_object = "GrimoireMetadata";
        Ok(Self {
            common_metadata: s.common_metadata.try_into()?,
            grimoire_name: normalizers::ensure_req_field_is_not_missing(
                s.grimoire_name,
                "grimoireName",
                parent_object,
            )?,
            grimoire_version: normalizers::ensure_req_field_is_not_missing(
                s.grimoire_version,
                "grimoireVersion",
                parent_object,
            )?,
            grimoire_description: s.grimoire_description,
            grimoire_authors: s.grimoire_authors,
            grimoire_source_code: s.grimoire_source_code,
            grimoire_website: s.grimoire_website,
            grimoire_documentation: s.grimoire_documentation,
            grimoire_readme: s.grimoire_readme,
            grimoire_license: normalizers::ensure_req_field_is_not_missing(
                s.grimoire_license,
                "grimoireLicense",
                parent_object,
            )?,
            grimoire_license_text: s.grimoire_license_text,
            grimoire_issue_tracker: s.grimoire_issue_tracker,
            grimoire_keywords: s.grimoire_keywords,
            grimoire_additional_metadata: s.grimoire_additional_metadata,
        })
    }
}

impl TryFrom<ParsedCoreContents> for NormalizedCoreContents {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedCoreContents) -> stdrt::Result<Self, Self::Error> {
        let parent_object = "CoreContents";
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.try_into()?,
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
            requires_confirmation: normalizers::ensure_req_field_is_not_missing(
                s.requires_confirmation,
                "requiresConfirmation",
                parent_object,
            )?,
        })
    }
}
