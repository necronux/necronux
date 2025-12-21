// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::UnifiedTool;
use crate::unified_grimoire_bundle_model::SchemaField;

#[derive(Debug)]
pub struct UnifiedSpell {
    pub name: SchemaField<String>,
    pub description: SchemaField<Option<String>>,
    pub requires_confirmation: bool,
    pub keywords: SchemaField<Vec<String>>,
    pub invocations: SchemaField<UnifiedSpellInvocations>,
}

#[derive(Debug)]
pub struct UnifiedSpellInvocations {
    pub cast: SchemaField<UnifiedInvocation>,
    pub affirm: SchemaField<UnifiedInvocation>,
    pub dispel: SchemaField<UnifiedInvocation>,
}

#[derive(Debug)]
pub struct UnifiedHex {
    pub name: SchemaField<String>,
    pub description: SchemaField<Option<String>>,
    pub requires_confirmation: bool,
    pub keywords: SchemaField<Vec<String>>,
    pub invocations: SchemaField<UnifiedHexInvocations>,
}

#[derive(Debug)]
pub struct UnifiedHexInvocations {
    pub lay: SchemaField<UnifiedInvocation>,
    pub discern: SchemaField<UnifiedInvocation>,
}

#[derive(Debug)]
pub struct UnifiedInvocation {
    pub prefix_args: SchemaField<Option<String>>,
    pub execution_command: SchemaField<String>,
    pub instrument_path: SchemaField<String>,
    pub tool: SchemaField<Vec<UnifiedTool>>,
}
