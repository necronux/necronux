// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{ParsedCoreContents, ParsedGrimoireMetadata};
use crate::grimoire_schemas::schemas::common_metadata;

pub fn grimoire_metadata() -> ParsedGrimoireMetadata {
    ParsedGrimoireMetadata {
        common_metadata: common_metadata::missing::schema_version_info(),
        grimoire_name: None,
        grimoire_version: None,
        grimoire_description: None,
        grimoire_authors: None,
        grimoire_source_code: None,
        grimoire_website: None,
        grimoire_documentation: None,
        grimoire_readme: None,
        grimoire_license: None,
        grimoire_license_text: None,
        grimoire_issue_tracker: None,
        grimoire_keywords: None,
        grimoire_additional_metadata: None,
    }
}

pub fn core_contents() -> ParsedCoreContents {
    ParsedCoreContents {
        grimoire_metadata: grimoire_metadata(),
        chapters: None,
        rituals: None,
        auto_perform_rituals: None,
        requires_confirmation: None,
    }
}
