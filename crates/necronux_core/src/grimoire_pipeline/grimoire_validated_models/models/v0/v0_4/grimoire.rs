// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{ValidatedChapter, ValidatedRitual};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ValidatedGrimoire {
    pub grimoire_metadata: Option<ValidatedGrimoireMetadata>,
    pub core_contents: ValidatedCoreContents,
}

#[derive(Debug)]
pub struct ValidatedGrimoireMetadata {
    pub grimoire_keywords: Option<Vec<String>>,
    pub grimoire_additional_metadata: Option<HashMap<String, HashMap<String, rpkl::Value>>>,
}

#[derive(Debug)]
pub struct ValidatedCoreContents {
    pub chapters: Option<HashMap<String, ValidatedChapter>>,
    pub rituals: Option<HashMap<String, ValidatedRitual>>,
    pub auto_perform_rituals: Option<Vec<rpkl::Value>>,
    pub requires_confirmation: Option<bool>,
}
