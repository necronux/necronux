// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{UnifiedGrimoireMetadata, UnifiedTool};
use crate::grimoire_unified_model::SchemaField;

#[derive(Debug)]
pub struct UnifiedSpell {
    pub grimoire_metadata: SchemaField<UnifiedGrimoireMetadata>,
    pub magic_type: SchemaField<String>,
    pub name: SchemaField<String>,
    pub description: SchemaField<Option<String>>,
    pub requires_confirmation: SchemaField<bool>,
    pub keywords: SchemaField<Option<Vec<String>>>,
    pub cast_invocation: SchemaField<UnifiedInvocation>,
    pub verify_invocation: SchemaField<UnifiedInvocation>,
    pub dispel_invocation: SchemaField<UnifiedInvocation>,
}

#[derive(Debug)]
pub struct UnifiedHex {
    pub grimoire_metadata: SchemaField<UnifiedGrimoireMetadata>,
    pub magic_type: SchemaField<String>,
    pub name: SchemaField<String>,
    pub description: SchemaField<Option<String>>,
    pub requires_confirmation: SchemaField<bool>,
    pub keywords: SchemaField<Option<Vec<String>>>,
    pub cast_invocation: SchemaField<UnifiedInvocation>,
    pub verify_invocation: SchemaField<UnifiedInvocation>,
}

#[derive(Debug)]
pub struct UnifiedInvocation {
    pub prefix_args: SchemaField<Option<String>>,
    pub execution_command: SchemaField<String>,
    pub instrument_path: SchemaField<String>,
    pub tool: SchemaField<Vec<UnifiedTool>>,
}
