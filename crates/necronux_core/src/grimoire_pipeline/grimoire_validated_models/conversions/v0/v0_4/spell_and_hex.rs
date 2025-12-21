// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::ValidateGrimoireError,
    grimoire_normalized_models::models::v0_4::{
        NormalizedHex, NormalizedHexInvocations, NormalizedInvocation, NormalizedSpell,
        NormalizedSpellInvocations,
    },
    grimoire_pipeline::validators,
    grimoire_validated_models::models::v0_4::{
        ValidatedHex, ValidatedHexInvocations, ValidatedInvocation, ValidatedSpell,
        ValidatedSpellInvocations,
    },
};

impl TryFrom<NormalizedSpell> for ValidatedSpell {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedSpell) -> Result<Self, Self::Error> {
        let parent_object = "Spell";
        Ok(Self {
            name: validators::ensure_str_is_not_empty(s.name, "name", parent_object)?,
            description: s
                .description
                .map(|d| validators::ensure_str_is_not_empty(d, "description", parent_object))
                .transpose()?,
            requires_confirmation: s.requires_confirmation,
            keywords: s
                .keywords
                .map(|k| {
                    k.into_iter()
                        .map(|kw| validators::ensure_str_is_not_empty(kw, "Keyword", parent_object))
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            invocations: s.invocations.try_into()?,
        })
    }
}

impl TryFrom<NormalizedSpellInvocations> for ValidatedSpellInvocations {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedSpellInvocations) -> Result<Self, Self::Error> {
        Ok(Self {
            cast: s.cast.try_into()?,
            affirm: s.affirm.try_into()?,
            dispel: s.dispel.try_into()?,
        })
    }
}

impl TryFrom<NormalizedHex> for ValidatedHex {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedHex) -> Result<Self, Self::Error> {
        let parent_object = "Hex";
        Ok(Self {
            name: validators::ensure_str_is_not_empty(s.name, "name", parent_object)?,
            description: s
                .description
                .map(|d| validators::ensure_str_is_not_empty(d, "description", parent_object))
                .transpose()?,
            requires_confirmation: s.requires_confirmation,
            keywords: s
                .keywords
                .map(|k| {
                    k.into_iter()
                        .map(|kw| validators::ensure_str_is_not_empty(kw, "Keyword", parent_object))
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            invocations: s.invocations.try_into()?,
        })
    }
}

impl TryFrom<NormalizedHexInvocations> for ValidatedHexInvocations {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedHexInvocations) -> Result<Self, Self::Error> {
        Ok(Self {
            lay: s.lay.try_into()?,
            discern: s.discern.try_into()?,
        })
    }
}

impl TryFrom<NormalizedInvocation> for ValidatedInvocation {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedInvocation) -> Result<Self, Self::Error> {
        let parent_object = "Invocation";
        Ok(Self {
            prefix_args: s
                .prefix_args
                .map(|p| validators::ensure_str_is_not_empty(p, "prefixArgs", parent_object))
                .transpose()?,
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
