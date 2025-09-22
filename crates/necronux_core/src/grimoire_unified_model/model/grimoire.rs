// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{UnifiedChapter, UnifiedRitual};
use crate::{
    grimoire_unified_model::SchemaField, models::common_metadata::ValidatedSchemaVersionInfo,
};
use semver::Version;
use std::collections::HashMap;

#[derive(Debug)]
pub struct UnifiedGrimoire {
    pub schema_version_info: SchemaField<ValidatedSchemaVersionInfo>,
    pub grimoire_metadata: SchemaField<UnifiedGrimoireMetadata>,
    pub core_contents: SchemaField<UnifiedCoreContents>,
}

#[derive(Debug)]
pub struct UnifiedGrimoireMetadata {
    pub schema_version_info: SchemaField<ValidatedSchemaVersionInfo>,
    pub grimoire_name: SchemaField<String>,
    pub grimoire_version: SchemaField<Version>,
    pub grimoire_description: SchemaField<Option<String>>,
    pub grimoire_authors: SchemaField<Option<Vec<String>>>,
    pub grimoire_source_code: SchemaField<Option<String>>,
    pub grimoire_website: SchemaField<Option<String>>,
    pub grimoire_documentation: SchemaField<Option<String>>,
    pub grimoire_readme: SchemaField<Option<String>>,
    pub grimoire_license: SchemaField<String>,
    pub grimoire_license_text: SchemaField<Option<String>>,
    pub grimoire_issue_tracker: SchemaField<Option<String>>,
    pub grimoire_keywords: SchemaField<Option<Vec<String>>>,
    pub grimoire_additional_metadata:
        SchemaField<Option<HashMap<String, HashMap<String, rpkl::Value>>>>,
}

#[derive(Debug)]
pub struct UnifiedCoreContents {
    pub grimoire_metadata: SchemaField<UnifiedGrimoireMetadata>,
    pub chapters: SchemaField<Option<HashMap<String, UnifiedChapter>>>,
    pub rituals: SchemaField<Option<HashMap<String, UnifiedRitual>>>,
    pub auto_perform_rituals: SchemaField<Option<Vec<rpkl::Value>>>,
    pub requires_confirmation: SchemaField<bool>,
}
