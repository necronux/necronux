// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{NormalizedChapter, NormalizedRitual};
use crate::grimoire_normalized_models::models::common_metadata;
use semver::Version;
use std::collections::HashMap;

#[derive(Debug)]
pub struct NormalizedGrimoire {
    pub common_metadata: common_metadata::NormalizedGrimoire,
    pub grimoire_metadata: NormalizedGrimoireMetadata,
    pub core_contents: NormalizedCoreContents,
}

#[derive(Debug)]
pub struct NormalizedGrimoireMetadata {
    pub common_metadata: common_metadata::NormalizedSchemaVersionInfo,
    pub grimoire_name: String,
    pub grimoire_version: Version,
    pub grimoire_description: Option<String>,
    pub grimoire_authors: Option<Vec<String>>,
    pub grimoire_source_code: Option<String>,
    pub grimoire_website: Option<String>,
    pub grimoire_documentation: Option<String>,
    pub grimoire_readme: Option<String>,
    pub grimoire_license: String,
    pub grimoire_license_text: Option<String>,
    pub grimoire_issue_tracker: Option<String>,
    pub grimoire_keywords: Option<Vec<String>>,
    pub grimoire_additional_metadata: Option<HashMap<String, HashMap<String, rpkl::Value>>>,
}

#[derive(Debug)]
pub struct NormalizedCoreContents {
    pub grimoire_metadata: NormalizedGrimoireMetadata,
    pub chapters: Option<HashMap<String, NormalizedChapter>>,
    pub rituals: Option<HashMap<String, NormalizedRitual>>,
    pub auto_perform_rituals: Option<Vec<rpkl::Value>>,
    pub requires_confirmation: bool,
}
