// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError,
    grimoire_normalized_models::{
        models::v0_4::{NormalizedHex, NormalizedInvocation, NormalizedSpell},
        normalizers,
    },
    grimoire_schemas::schemas::v0_4::{ParsedHex, ParsedInvocation, ParsedSpell},
};

impl TryFrom<ParsedSpell> for NormalizedSpell {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedSpell) -> Result<Self, Self::Error> {
        let parent_object = "Spell";
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            magic_type: normalizers::ensure_req_field_is_not_missing(
                s.magic_type,
                "magicType",
                parent_object,
            )?,
            name: normalizers::ensure_req_field_is_not_missing(s.name, "name", parent_object)?,
            description: s.description,
            requires_confirmation: normalizers::ensure_req_field_is_not_missing(
                s.requires_confirmation,
                "requiresConfirmation",
                parent_object,
            )?,
            keywords: s.keywords,
            cast_invocation: normalizers::ensure_req_field_is_not_missing(
                s.cast_invocation,
                "castInvocation",
                parent_object,
            )?
            .try_into()?,
            verify_invocation: normalizers::ensure_req_field_is_not_missing(
                s.verify_invocation,
                "verifyInvocation",
                parent_object,
            )?
            .try_into()?,
            dispel_invocation: normalizers::ensure_req_field_is_not_missing(
                s.dispel_invocation,
                "dispelInvocation",
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
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            magic_type: normalizers::ensure_req_field_is_not_missing(
                s.magic_type,
                "magicType",
                parent_object,
            )?,
            name: normalizers::ensure_req_field_is_not_missing(s.name, "name", parent_object)?,
            description: s.description,
            requires_confirmation: normalizers::ensure_req_field_is_not_missing(
                s.requires_confirmation,
                "requiresConfirmation",
                parent_object,
            )?,
            keywords: s.keywords,
            cast_invocation: normalizers::ensure_req_field_is_not_missing(
                s.cast_invocation,
                "castInvocation",
                parent_object,
            )?
            .try_into()?,
            verify_invocation: normalizers::ensure_req_field_is_not_missing(
                s.verify_invocation,
                "verifyInvocation",
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
