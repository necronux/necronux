// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::ValidateGrimoireError,
    grimoire_normalized_models::models::v0_4::{
        NormalizedHex, NormalizedInvocation, NormalizedSpell,
    },
    models::v0_4::{ValidatedHex, ValidatedInvocation, ValidatedSpell},
    validators,
};

impl TryFrom<NormalizedSpell> for ValidatedSpell {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedSpell) -> Result<Self, Self::Error> {
        let parent_object = "Spell";
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            magic_type: {
                let mt =
                    validators::ensure_str_is_not_empty(s.magic_type, "magicType", parent_object)?;

                match mt.as_str() {
                    "spell" | "hex" => mt,
                    _ => {
                        return Err(ValidateGrimoireError::InvalidFieldValue {
                            field_name: "magicType".to_string(),
                            value: mt.clone(),
                            parent_object_name: parent_object.to_string(),
                            reason: "must be either 'spell' or 'hex'".to_string(),
                        });
                    }
                }
            },
            name: validators::ensure_str_is_not_empty(s.name, "name", parent_object)?,
            description: validators::ensure_some_str_is_not_empty(
                s.description,
                "description",
                parent_object,
            )?,
            requires_confirmation: s.requires_confirmation,
            keywords: validators::ensure_some_vec_is_not_empty(
                s.keywords,
                "keywords",
                parent_object,
            )?,
            cast_invocation: s.cast_invocation.try_into()?,
            verify_invocation: s.verify_invocation.try_into()?,
            dispel_invocation: s.dispel_invocation.try_into()?,
        })
    }
}

impl TryFrom<NormalizedHex> for ValidatedHex {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedHex) -> Result<Self, Self::Error> {
        let parent_object = "Hex";
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            magic_type: {
                let mt =
                    validators::ensure_str_is_not_empty(s.magic_type, "magicType", parent_object)?;

                match mt.as_str() {
                    "spell" | "hex" => mt,
                    _ => {
                        return Err(ValidateGrimoireError::InvalidFieldValue {
                            field_name: "magicType".to_string(),
                            value: mt.clone(),
                            parent_object_name: parent_object.to_string(),
                            reason: "must be either 'spell' or 'hex'".to_string(),
                        });
                    }
                }
            },
            name: validators::ensure_str_is_not_empty(s.name, "name", parent_object)?,
            description: validators::ensure_some_str_is_not_empty(
                s.description,
                "description",
                parent_object,
            )?,
            requires_confirmation: s.requires_confirmation,
            keywords: validators::ensure_some_vec_is_not_empty(
                s.keywords,
                "keywords",
                parent_object,
            )?,
            cast_invocation: s.cast_invocation.try_into()?,
            verify_invocation: s.verify_invocation.try_into()?,
        })
    }
}

impl TryFrom<NormalizedInvocation> for ValidatedInvocation {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedInvocation) -> Result<Self, Self::Error> {
        let parent_object = "Invocation";
        Ok(Self {
            prefix_args: validators::ensure_some_str_is_not_empty(
                s.prefix_args,
                "prefixArgs",
                parent_object,
            )?,
            execution_command: validators::ensure_str_is_not_empty(
                s.execution_command,
                "executionCommand",
                parent_object,
            )?,
            instrument_path: validators::ensure_str_is_not_empty(
                s.instrument_path,
                "instrumentPath",
                parent_object,
            )?,
            tool: validators::ensure_vec_is_not_empty(s.tool, "tool", parent_object)?
                .into_iter()
                .map(|t| t.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
