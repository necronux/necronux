// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use std::collections::HashMap;

#[derive(Debug)]
pub struct UnifiedGrimoire {
    pub std_schema_version: Option<String>,
    pub grimoire_metadata: Option<UnifiedGrimoireMetadata>,
}

#[derive(Debug)]
pub struct UnifiedGrimoireMetadata {
    pub std_schema_version: Option<String>,
    pub grimoire_name: Option<String>,
    pub grimoire_version: Option<String>,
    pub grimoire_description: Option<Option<String>>,
    pub grimoire_authors: Option<Option<Vec<String>>>,
    pub grimoire_source_code: Option<Option<String>>,
    pub grimoire_website: Option<Option<String>>,
    pub grimoire_documentation: Option<Option<String>>,
    pub grimoire_readme: Option<Option<String>>,
    pub grimoire_license: Option<String>,
    pub grimoire_license_text: Option<Option<String>>,
    pub grimoire_issue_tracker: Option<Option<String>>,
    pub grimoire_keywords: Option<Option<Vec<String>>>,
    pub grimoire_additional_metadata: Option<Option<HashMap<String, HashMap<String, rpkl::Value>>>>,
}
