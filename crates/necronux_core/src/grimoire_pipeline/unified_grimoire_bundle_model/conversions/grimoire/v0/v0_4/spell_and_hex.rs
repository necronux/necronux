// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    grimoire_validated_models::models::v0_4::{
        ValidatedHex, ValidatedHexInvocations, ValidatedInvocation, ValidatedSpell,
        ValidatedSpellInvocations,
    },
    unified_grimoire_bundle_model::{
        model::{
            UnifiedHex, UnifiedHexInvocations, UnifiedInvocation, UnifiedSpell,
            UnifiedSpellInvocations,
        },
        SchemaField,
    },
};

impl From<ValidatedSpell> for UnifiedSpell {
    fn from(s: ValidatedSpell) -> Self {
        Self {
            name: SchemaField::Present(s.name),
            description: SchemaField::Present(s.description),
            requires_confirmation: s.requires_confirmation.unwrap_or(false),
            keywords: SchemaField::Present(s.keywords.unwrap_or_default()),
            invocations: SchemaField::Present(s.invocations.into()),
        }
    }
}

impl From<ValidatedSpellInvocations> for UnifiedSpellInvocations {
    fn from(s: ValidatedSpellInvocations) -> Self {
        Self {
            cast: SchemaField::Present(s.cast.into()),
            affirm: SchemaField::Present(s.affirm.into()),
            dispel: SchemaField::Present(s.dispel.into()),
        }
    }
}

impl From<ValidatedHex> for UnifiedHex {
    fn from(s: ValidatedHex) -> Self {
        Self {
            name: SchemaField::Present(s.name),
            description: SchemaField::Present(s.description),
            requires_confirmation: s.requires_confirmation.unwrap_or(false),
            keywords: SchemaField::Present(s.keywords.unwrap_or_default()),
            invocations: SchemaField::Present(s.invocations.into()),
        }
    }
}

impl From<ValidatedHexInvocations> for UnifiedHexInvocations {
    fn from(s: ValidatedHexInvocations) -> Self {
        Self {
            lay: SchemaField::Present(s.lay.into()),
            discern: SchemaField::Present(s.discern.into()),
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
