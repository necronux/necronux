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
pub struct Grimoire {
    #[serde(flatten)]
    pub minimal: Minimal,

    #[serde(rename = "grimoireMetadata")]
    pub grimoire_metadata: GrimoireMetadata,

    #[serde(rename = "coreContents")]
    pub core_contents: CoreContents,

    pub sigils: Option<HashMap<String, String>>,

    pub enchantments: Enchantments,
}

#[derive(Debug, Deserialize)]
pub struct GrimoireMetadata {
    #[serde(rename = "stdSchemaVersion")]
    pub std_schema_version: String,

    #[serde(rename = "grimoireName")]
    pub grimoire_name: String,

    #[serde(rename = "grimoireVersion")]
    pub grimoire_version: String,

    #[serde(rename = "grimoireDescription")]
    pub grimoire_description: Option<String>,

    #[serde(rename = "grimoireAuthors")]
    pub grimoire_authors: Option<Vec<String>>,

    #[serde(rename = "grimoireSourceCode")]
    pub grimoire_source_code: Option<String>,

    #[serde(rename = "grimoireWebsite")]
    pub grimoire_website: Option<String>,

    #[serde(rename = "grimoireDocumentation")]
    pub grimoire_documentation: Option<String>,

    #[serde(rename = "grimoireReadme")]
    pub grimoire_readme: Option<String>,

    #[serde(rename = "grimoireLicense")]
    pub grimoire_license: String,

    #[serde(rename = "grimoireLicenseText")]
    pub grimoire_license_text: Option<String>,

    #[serde(rename = "grimoireIssueTracker")]
    pub grimoire_issue_tracker: Option<String>,

    #[serde(rename = "grimoireKeywords")]
    pub grimoire_keywords: Option<Vec<String>>,

    #[serde(rename = "grimoireAdditionalMetadata")]
    pub grimoire_additional_metadata: Option<HashMap<String, HashMap<String, rpkl::Value>>>,
}

#[derive(Debug, Deserialize)]
pub struct CoreContents {
    #[serde(flatten)]
    pub metadata: GrimoireMetadata,

    pub chapters: HashMap<String, Chapter>,

    pub rituals: Option<HashMap<String, Ritual>>,

    #[serde(rename = "compositeRituals")]
    pub composite_rituals: Option<HashMap<String, CompositeRitual>>,

    #[serde(rename = "autoPerformRituals")]
    pub auto_perform_rituals: Option<Vec<rpkl::Value>>,

    #[serde(rename = "linkedGrimoires")]
    pub linked_grimoires: Option<HashMap<String, LinkedGrimoire>>,

    #[serde(rename = "requiresConfirmation")]
    pub requires_confirmation: bool,
}

#[derive(Debug, Deserialize)]
pub struct LinkedGrimoire {
    #[serde(rename = "coreContents")]
    pub core_contents: Option<rpkl::Value>,

    pub chapters: Option<HashMap<String, rpkl::Value>>,

    pub rituals: Option<HashMap<String, rpkl::Value>>,

    #[serde(rename = "compositeRituals")]
    pub composite_rituals: Option<HashMap<String, rpkl::Value>>,

    pub spells: Option<HashMap<String, rpkl::Value>>,

    pub hexes: Option<HashMap<String, rpkl::Value>>,

    #[serde(rename = "requiresConfirmation")]
    pub requires_confirmation: bool,
}
