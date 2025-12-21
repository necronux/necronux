// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError,
    grimoire_normalized_models::models::v0_4::{
        NormalizedHex, NormalizedHexInvocations, NormalizedInvocation, NormalizedSpell,
        NormalizedSpellInvocations,
    },
    grimoire_parsed_models::models::v0_4::{
        ParsedHex, ParsedHexInvocations, ParsedInvocation, ParsedSpell, ParsedSpellInvocations,
    },
    grimoire_pipeline::normalizers,
};

impl TryFrom<ParsedSpell> for NormalizedSpell {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedSpell) -> Result<Self, Self::Error> {
        let parent_object = "Spell";
        Ok(Self {
            name: normalizers::ensure_req_field_is_not_missing(s.name, "name", parent_object)?,
            description: s.description,
            requires_confirmation: s.requires_confirmation,
            keywords: s.keywords,
            invocations: normalizers::ensure_req_field_is_not_missing(
                s.invocations,
                "invocations",
                parent_object,
            )?
            .try_into()?,
        })
    }
}

impl TryFrom<ParsedSpellInvocations> for NormalizedSpellInvocations {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedSpellInvocations) -> Result<Self, Self::Error> {
        let parent_object = "SpellInvocations";
        Ok(Self {
            cast: normalizers::ensure_req_field_is_not_missing(s.cast, "cast", parent_object)?
                .try_into()?,
            affirm: normalizers::ensure_req_field_is_not_missing(
                s.affirm,
                "affirm",
                parent_object,
            )?
            .try_into()?,
            dispel: normalizers::ensure_req_field_is_not_missing(
                s.dispel,
                "dispel",
                parent_object,
            )?
            .try_into()?,
        })
    }
}

impl TryFrom<ParsedHex> for NormalizedHex {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedHex) -> Result<Self, Self::Error> {
        let parent_object = "Hex";
        Ok(Self {
            name: normalizers::ensure_req_field_is_not_missing(s.name, "name", parent_object)?,
            description: s.description,
            requires_confirmation: s.requires_confirmation,
            keywords: s.keywords,
            invocations: normalizers::ensure_req_field_is_not_missing(
                s.invocations,
                "invocations",
                parent_object,
            )?
            .try_into()?,
        })
    }
}

impl TryFrom<ParsedHexInvocations> for NormalizedHexInvocations {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedHexInvocations) -> Result<Self, Self::Error> {
        let parent_object = "HexInvocations";
        Ok(Self {
            lay: normalizers::ensure_req_field_is_not_missing(s.lay, "lay", parent_object)?
                .try_into()?,
            discern: normalizers::ensure_req_field_is_not_missing(
                s.discern,
                "discern",
                parent_object,
            )?
            .try_into()?,
        })
    }
}

impl TryFrom<ParsedInvocation> for NormalizedInvocation {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedInvocation) -> Result<Self, Self::Error> {
        let parent_object = "Invocation";
        Ok(Self {
            prefix_args: s.prefix_args,
            execution_command: normalizers::ensure_req_field_is_not_missing(
                s.execution_command,
                "executionCommand",
                parent_object,
            )?,
            instrument_path: normalizers::ensure_req_field_is_not_missing(
                s.instrument_path,
                "instrumentPath",
                parent_object,
            )?,
            tool: normalizers::ensure_req_field_is_not_missing(s.tool, "tool", parent_object)?
                .into_iter()
                .map(|t| t.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
