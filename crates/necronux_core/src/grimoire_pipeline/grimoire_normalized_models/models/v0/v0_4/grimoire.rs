// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{NormalizedChapter, NormalizedRitual};
use std::collections::HashMap;

#[derive(Debug)]
pub struct NormalizedGrimoire {
    pub grimoire_metadata: Option<NormalizedGrimoireMetadata>,
    pub core_contents: NormalizedCoreContents,
}

#[derive(Debug)]
pub struct NormalizedGrimoireMetadata {
    pub grimoire_keywords: Option<Vec<String>>,
    pub grimoire_additional_metadata: Option<HashMap<String, HashMap<String, rpkl::Value>>>,
}

#[derive(Debug)]
pub struct NormalizedCoreContents {
    pub chapters: Option<HashMap<String, NormalizedChapter>>,
    pub rituals: Option<HashMap<String, NormalizedRitual>>,
    pub auto_perform_rituals: Option<Vec<rpkl::Value>>,
    pub requires_confirmation: Option<bool>,
}
