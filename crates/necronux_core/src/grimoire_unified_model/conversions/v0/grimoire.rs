// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    grimoire_unified_model::{
        SchemaField,
        model::{UnifiedCoreContents, UnifiedGrimoire, UnifiedGrimoireMetadata},
    },
    models::v0::{ValidatedCoreContents, ValidatedGrimoire, ValidatedGrimoireMetadata},
};
use std::collections::HashMap;

impl From<ValidatedGrimoire> for UnifiedGrimoire {
    fn from(s: ValidatedGrimoire) -> Self {
        Self {
            schema_version_info: SchemaField::Present(s.common_metadata.schema_version_info),
            grimoire_metadata: SchemaField::Present(s.grimoire_metadata.into()),
            core_contents: SchemaField::Present(s.core_contents.into()),
        }
    }
}

impl From<ValidatedGrimoireMetadata> for UnifiedGrimoireMetadata {
    fn from(s: ValidatedGrimoireMetadata) -> Self {
        Self {
            schema_version_info: SchemaField::Present(s.common_metadata),
            grimoire_name: SchemaField::Present(s.grimoire_name),
            grimoire_version: SchemaField::Present(s.grimoire_version),
            grimoire_description: SchemaField::Present(s.grimoire_description),
            grimoire_authors: SchemaField::Present(s.grimoire_authors),
            grimoire_source_code: SchemaField::Present(s.grimoire_source_code),
            grimoire_website: SchemaField::Present(s.grimoire_website),
            grimoire_documentation: SchemaField::Present(s.grimoire_documentation),
            grimoire_readme: SchemaField::Present(s.grimoire_readme),
            grimoire_license: SchemaField::Present(s.grimoire_license),
            grimoire_license_text: SchemaField::Present(s.grimoire_license_text),
            grimoire_issue_tracker: SchemaField::Present(s.grimoire_issue_tracker),
            grimoire_keywords: SchemaField::Present(s.grimoire_keywords),
            grimoire_additional_metadata: SchemaField::Present(s.grimoire_additional_metadata),
        }
    }
}

impl From<ValidatedCoreContents> for UnifiedCoreContents {
    fn from(s: ValidatedCoreContents) -> Self {
        Self {
            grimoire_metadata: SchemaField::Present(s.grimoire_metadata.into()),
            chapters: SchemaField::Present(s.chapters.map(|c| {
                c.into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect::<HashMap<_, _>>()
            })),
            rituals: SchemaField::Present(s.rituals.map(|r| {
                r.into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect::<HashMap<_, _>>()
            })),
            auto_perform_rituals: SchemaField::Present(s.auto_perform_rituals),
            requires_confirmation: SchemaField::Present(s.requires_confirmation),
        }
    }
}
