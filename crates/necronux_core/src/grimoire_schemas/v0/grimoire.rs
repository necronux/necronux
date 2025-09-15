// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{Chapter, CompositeRitual, Enchantments, Ritual};
use crate::grimoire_schemas::minimal_metadata::Grimoire as Minimal;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grimoire {
    #[serde(flatten)]
    pub minimal: Minimal,

    pub grimoire_metadata: GrimoireMetadata,

    pub core_contents: CoreContents,

    pub sigils: Option<HashMap<String, String>>,

    pub enchantments: Enchantments,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrimoireMetadata {
    pub std_schema_version: String,

    pub grimoire_name: String,

    pub grimoire_version: String,

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreContents {
    #[serde(flatten)]
    pub metadata: GrimoireMetadata,

    pub chapters: HashMap<String, Chapter>,

    pub rituals: Option<HashMap<String, Ritual>>,

    pub composite_rituals: Option<HashMap<String, CompositeRitual>>,

    pub auto_perform_rituals: Option<Vec<rpkl::Value>>,

    pub linked_grimoires: Option<HashMap<String, LinkedGrimoire>>,

    pub requires_confirmation: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedGrimoire {
    pub core_contents: Option<rpkl::Value>,

    pub chapters: Option<HashMap<String, rpkl::Value>>,

    pub rituals: Option<HashMap<String, rpkl::Value>>,

    pub composite_rituals: Option<HashMap<String, rpkl::Value>>,

    pub spells: Option<HashMap<String, rpkl::Value>>,

    pub hexes: Option<HashMap<String, rpkl::Value>>,

    pub requires_confirmation: bool,
}
