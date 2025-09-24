// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError,
    grimoire_normalized_models::models::v0_4::{
        NormalizedHex, NormalizedInvocation, NormalizedSpell,
    },
    grimoire_schemas::schemas::v0_4::{ParsedHex, ParsedInvocation, ParsedSpell},
};

impl TryFrom<ParsedSpell> for NormalizedSpell {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedSpell) -> Result<Self, Self::Error> {
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            magic_type: s.magic_type.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "magicType".to_string(),
                    parent_object_name: "Spell".to_string(),
                }
            })?,
            name: s
                .name
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "name".to_string(),
                    parent_object_name: "Spell".to_string(),
                })?,
            description: s.description,
            requires_confirmation: s.requires_confirmation.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "requiresConfirmation".to_string(),
                    parent_object_name: "Spell".to_string(),
                }
            })?,
            keywords: s.keywords,
            cast_invocation: s
                .cast_invocation
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "castInvocation".to_string(),
                    parent_object_name: "Spell".to_string(),
                })?
                .try_into()?,
            verify_invocation: s
                .verify_invocation
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "verifyInvocation".to_string(),
                    parent_object_name: "Spell".to_string(),
                })?
                .try_into()?,
            dispel_invocation: s
                .dispel_invocation
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "dispelInvocation".to_string(),
                    parent_object_name: "Spell".to_string(),
                })?
                .try_into()?,
        })
    }
}

impl TryFrom<ParsedHex> for NormalizedHex {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedHex) -> Result<Self, Self::Error> {
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            magic_type: s.magic_type.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "magicType".to_string(),
                    parent_object_name: "Spell".to_string(),
                }
            })?,
            name: s
                .name
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "name".to_string(),
                    parent_object_name: "Hex".to_string(),
                })?,
            description: s.description,
            requires_confirmation: s.requires_confirmation.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "requiresConfirmation".to_string(),
                    parent_object_name: "Hex".to_string(),
                }
            })?,
            keywords: s.keywords,
            cast_invocation: s
                .cast_invocation
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "castInvocation".to_string(),
                    parent_object_name: "Hex".to_string(),
                })?
                .try_into()?,
            verify_invocation: s
                .verify_invocation
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "verifyInvocation".to_string(),
                    parent_object_name: "Hex".to_string(),
                })?
                .try_into()?,
        })
    }
}

impl TryFrom<ParsedInvocation> for NormalizedInvocation {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedInvocation) -> Result<Self, Self::Error> {
        Ok(Self {
            prefix_args: s.prefix_args,
            execution_command: s.execution_command.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "executionCommand".to_string(),
                    parent_object_name: "Invocation".to_string(),
                }
            })?,
            instrument_path: s.instrument_path.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "instrumentPath".to_string(),
                    parent_object_name: "Invocation".to_string(),
                }
            })?,
            tool: s
                .tool
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "tool".to_string(),
                    parent_object_name: "Invocation".to_string(),
                })?
                .into_iter()
                .map(|t| t.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
