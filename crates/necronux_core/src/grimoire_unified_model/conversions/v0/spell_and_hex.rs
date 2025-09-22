// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    grimoire_unified_model::{
        SchemaField,
        model::{UnifiedHex, UnifiedInvocation, UnifiedSpell},
    },
    models::v0::{ValidatedHex, ValidatedInvocation, ValidatedSpell},
};

impl From<ValidatedSpell> for UnifiedSpell {
    fn from(s: ValidatedSpell) -> Self {
        Self {
            grimoire_metadata: SchemaField::Present(s.grimoire_metadata.into()),
            magic_type: SchemaField::Present(s.magic_type),
            name: SchemaField::Present(s.name),
            description: SchemaField::Present(s.description),
            requires_confirmation: SchemaField::Present(s.requires_confirmation),
            keywords: SchemaField::Present(s.keywords),
            cast_invocation: SchemaField::Present(s.cast_invocation.into()),
            verify_invocation: SchemaField::Present(s.verify_invocation.into()),
            dispel_invocation: SchemaField::Present(s.dispel_invocation.into()),
        }
    }
}

impl From<ValidatedHex> for UnifiedHex {
    fn from(s: ValidatedHex) -> Self {
        Self {
            grimoire_metadata: SchemaField::Present(s.grimoire_metadata.into()),
            magic_type: SchemaField::Present(s.magic_type),
            name: SchemaField::Present(s.name),
            description: SchemaField::Present(s.description),
            requires_confirmation: SchemaField::Present(s.requires_confirmation),
            keywords: SchemaField::Present(s.keywords),
            cast_invocation: SchemaField::Present(s.cast_invocation.into()),
            verify_invocation: SchemaField::Present(s.verify_invocation.into()),
        }
    }
}

impl From<ValidatedInvocation> for UnifiedInvocation {
    fn from(s: ValidatedInvocation) -> Self {
        Self {
            prefix_args: SchemaField::Present(s.prefix_args),
            execution_command: SchemaField::Present(s.execution_command),
            instrument_path: SchemaField::Present(s.instrument_path),
            tool: SchemaField::Present(s.tool.into_iter().map(|t| t.into()).collect()),
        }
    }
}
