// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError,
    grimoire_normalized_models::models::v0::{
        NormalizedCoreContents, NormalizedGrimoire, NormalizedGrimoireMetadata,
    },
    grimoire_schemas::schemas::v0::{ParsedCoreContents, ParsedGrimoire, ParsedGrimoireMetadata},
};
use std::{collections::HashMap, result as stdrt};

impl TryFrom<ParsedGrimoire> for NormalizedGrimoire {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedGrimoire) -> stdrt::Result<Self, Self::Error> {
        Ok(Self {
            common_metadata: s.common_metadata.try_into()?,
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            core_contents: s.core_contents.try_into()?,
        })
    }
}

impl TryFrom<ParsedGrimoireMetadata> for NormalizedGrimoireMetadata {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedGrimoireMetadata) -> stdrt::Result<Self, Self::Error> {
        Ok(Self {
            common_metadata: s.common_metadata.try_into()?,
            grimoire_name: s.grimoire_name.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "GrimoireMetadata.grimoireName".to_string(),
                }
            })?,
            grimoire_version: s.grimoire_version.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "GrimoireMetadata.grimoireVersion".to_string(),
                }
            })?,
            grimoire_description: s.grimoire_description,
            grimoire_authors: s.grimoire_authors,
            grimoire_source_code: s.grimoire_source_code,
            grimoire_website: s.grimoire_website,
            grimoire_documentation: s.grimoire_documentation,
            grimoire_readme: s.grimoire_readme,
            grimoire_license: s.grimoire_license.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "GrimoireMetadata.grimoireLicense".to_string(),
                }
            })?,
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
            requires_confirmation: s.requires_confirmation.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "CoreContents.requiresConfirmation".to_string(),
                }
            })?,
        })
    }
}
