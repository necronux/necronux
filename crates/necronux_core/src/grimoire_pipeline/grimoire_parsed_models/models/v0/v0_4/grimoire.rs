// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{ParsedChapter, ParsedRitual};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedGrimoire {
    pub grimoire_metadata: Option<ParsedGrimoireMetadata>,

    // Option as RFB (See lib.rs)
    pub core_contents: Option<ParsedCoreContents>,
    /* These are on hold
    pub sigils: Option<HashMap<String, String>>,

    pub enchantments: Option<ParsedEnchantments>,
    */
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedGrimoireMetadata {
    pub grimoire_keywords: Option<Vec<String>>,

    pub grimoire_additional_metadata: Option<HashMap<String, HashMap<String, rpkl::Value>>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedCoreContents {
    pub chapters: Option<HashMap<String, ParsedChapter>>,

    pub rituals: Option<HashMap<String, ParsedRitual>>,

    /* These are on hold
    pub composite_rituals: Option<HashMap<String, ParsedCompositeRitual>>,
    */
    pub auto_perform_rituals: Option<Vec<rpkl::Value>>,

    /* These are on hold
    pub linked_grimoires: Option<HashMap<String, ParsedLinkedGrimoire>>,
    */
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
