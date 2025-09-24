// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{ParsedChapter, ParsedRitual};
use crate::grimoire_schemas::schemas::common_metadata;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedGrimoire {
    #[serde(flatten)]
    pub common_metadata: common_metadata::ParsedGrimoire,

    // Option as RFB (See mod.rs)
    pub grimoire_metadata: Option<ParsedGrimoireMetadata>,

    // Option as RFB (See mod.rs)
    pub core_contents: Option<ParsedCoreContents>,
    /* These are on hold
    pub sigils: Option<HashMap<String, String>>,

    pub enchantments: Option<ParsedEnchantments>,
    */
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedGrimoireMetadata {
    #[serde(flatten)]
    pub common_metadata: common_metadata::ParsedSchemaVersionInfo,

    // Option as RFB (See mod.rs)
    pub grimoire_name: Option<String>,

    // Option as RFB (See mod.rs)
    pub grimoire_version: Option<String>,

    pub grimoire_description: Option<String>,

    pub grimoire_authors: Option<Vec<String>>,

    pub grimoire_source_code: Option<String>,

    pub grimoire_website: Option<String>,

    pub grimoire_documentation: Option<String>,

    pub grimoire_readme: Option<String>,

    // Option as RFB (See mod.rs)
    pub grimoire_license: Option<String>,

    pub grimoire_license_text: Option<String>,

    pub grimoire_issue_tracker: Option<String>,

    pub grimoire_keywords: Option<Vec<String>>,

    pub grimoire_additional_metadata: Option<HashMap<String, HashMap<String, rpkl::Value>>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedCoreContents {
    #[serde(flatten)]
    pub grimoire_metadata: ParsedGrimoireMetadata,

    pub chapters: Option<HashMap<String, ParsedChapter>>,

    pub rituals: Option<HashMap<String, ParsedRitual>>,

    /* These are on hold
    pub composite_rituals: Option<HashMap<String, ParsedCompositeRitual>>,
    */
    pub auto_perform_rituals: Option<Vec<rpkl::Value>>,

    /* These are on hold
    pub linked_grimoires: Option<HashMap<String, ParsedLinkedGrimoire>>,
    */
    // Option as RFB (See mod.rs)
    pub requires_confirmation: Option<bool>,
}

/* These are on hold
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedLinkedGrimoire {
    pub core_contents: Option<rpkl::Value>,

    pub chapters: Option<HashMap<String, rpkl::Value>>,

    pub rituals: Option<HashMap<String, rpkl::Value>>,

    pub composite_rituals: Option<HashMap<String, rpkl::Value>>,

    pub spells: Option<HashMap<String, rpkl::Value>>,

    pub hexes: Option<HashMap<String, rpkl::Value>>,

    pub requires_confirmation: bool,
}
*/
