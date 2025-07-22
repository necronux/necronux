// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{UnifiedGrimoire, UnifiedGrimoireMetadata};

impl From<crate::grimoire_schemas::v0::Grimoire> for UnifiedGrimoire {
    fn from(v0: crate::grimoire_schemas::v0::Grimoire) -> Self {
        Self {
            std_schema_version: Some(v0.minimal.std_schema_version.to_string()),
            grimoire_metadata: Some(v0.grimoire_metadata.into()),
        }
    }
}

impl From<crate::grimoire_schemas::v0::GrimoireMetadata> for UnifiedGrimoireMetadata {
    fn from(v0: crate::grimoire_schemas::v0::GrimoireMetadata) -> Self {
        Self {
            std_schema_version: Some(v0.std_schema_version),
            grimoire_name: Some(v0.grimoire_name),
            grimoire_version: Some(v0.grimoire_version),
            grimoire_description: Some(v0.grimoire_description),
            grimoire_authors: Some(v0.grimoire_authors),
            grimoire_source_code: Some(v0.grimoire_source_code),
            grimoire_website: Some(v0.grimoire_website),
            grimoire_documentation: Some(v0.grimoire_documentation),
            grimoire_readme: Some(v0.grimoire_readme),
            grimoire_license: Some(v0.grimoire_license),
            grimoire_license_text: Some(v0.grimoire_license_text),
            grimoire_issue_tracker: Some(v0.grimoire_issue_tracker),
            grimoire_keywords: Some(v0.grimoire_keywords),
            grimoire_additional_metadata: Some(v0.grimoire_additional_metadata),
        }
    }
}
