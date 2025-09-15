// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{UnifiedGrimoire, UnifiedGrimoireMetadata};
use crate::grimoire_schemas::v0;

impl From<v0::Grimoire> for UnifiedGrimoire {
    fn from(s: v0::Grimoire) -> Self {
        Self {
            std_schema_version: Some(s.minimal.std_schema_version.to_string()),
            grimoire_metadata: Some(s.grimoire_metadata.into()),
        }
    }
}

impl From<v0::GrimoireMetadata> for UnifiedGrimoireMetadata {
    fn from(s: v0::GrimoireMetadata) -> Self {
        Self {
            std_schema_version: Some(s.std_schema_version),
            grimoire_name: Some(s.grimoire_name),
            grimoire_version: Some(s.grimoire_version),
            grimoire_description: Some(s.grimoire_description),
            grimoire_authors: Some(s.grimoire_authors),
            grimoire_source_code: Some(s.grimoire_source_code),
            grimoire_website: Some(s.grimoire_website),
            grimoire_documentation: Some(s.grimoire_documentation),
            grimoire_readme: Some(s.grimoire_readme),
            grimoire_license: Some(s.grimoire_license),
            grimoire_license_text: Some(s.grimoire_license_text),
            grimoire_issue_tracker: Some(s.grimoire_issue_tracker),
            grimoire_keywords: Some(s.grimoire_keywords),
            grimoire_additional_metadata: Some(s.grimoire_additional_metadata),
        }
    }
}
