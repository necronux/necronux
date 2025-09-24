// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{ValidatedChapter, ValidatedRitual};
use crate::grimoire_validated_models::models::common_metadata;
use semver::Version;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ValidatedGrimoire {
    pub common_metadata: common_metadata::ValidatedGrimoire,
    pub grimoire_metadata: ValidatedGrimoireMetadata,
    pub core_contents: ValidatedCoreContents,
}

#[derive(Debug)]
pub struct ValidatedGrimoireMetadata {
    pub common_metadata: common_metadata::ValidatedSchemaVersionInfo,
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
pub struct ValidatedCoreContents {
    pub grimoire_metadata: ValidatedGrimoireMetadata,
    pub chapters: Option<HashMap<String, ValidatedChapter>>,
    pub rituals: Option<HashMap<String, ValidatedRitual>>,
    pub auto_perform_rituals: Option<Vec<rpkl::Value>>,
    pub requires_confirmation: bool,
}
