// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{UnifiedChapter, UnifiedRitual};
use crate::unified_grimoire_bundle_model::SchemaField;
use std::collections::HashMap;

#[derive(Debug)]
pub struct UnifiedGrimoire {
    pub grimoire_metadata: SchemaField<Option<UnifiedGrimoireMetadata>>,
    pub core_contents: SchemaField<UnifiedCoreContents>,
}

#[derive(Debug)]
pub struct UnifiedGrimoireMetadata {
    pub grimoire_keywords: SchemaField<Vec<String>>,
    pub grimoire_additional_metadata: SchemaField<HashMap<String, HashMap<String, rpkl::Value>>>,
}

#[derive(Debug)]
pub struct UnifiedCoreContents {
    pub chapters: SchemaField<HashMap<String, UnifiedChapter>>,
    pub rituals: SchemaField<HashMap<String, UnifiedRitual>>,
    pub auto_perform_rituals: SchemaField<Vec<rpkl::Value>>,
    pub requires_confirmation: bool,
}
