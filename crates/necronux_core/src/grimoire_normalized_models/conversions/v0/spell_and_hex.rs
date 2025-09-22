// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError,
    grimoire_normalized_models::models::v0::{
        NormalizedHex, NormalizedInvocation, NormalizedSpell,
    },
    grimoire_schemas::schemas::v0::{ParsedHex, ParsedInvocation, ParsedSpell},
};

impl TryFrom<ParsedSpell> for NormalizedSpell {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedSpell) -> Result<Self, Self::Error> {
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            magic_type: s.magic_type,
            name: s.name,
            description: s.description,
            requires_confirmation: s.requires_confirmation,
            keywords: s.keywords,
            cast_invocation: s.cast_invocation.try_into()?,
            verify_invocation: s.verify_invocation.try_into()?,
            dispel_invocation: s.dispel_invocation.try_into()?,
        })
    }
}

impl TryFrom<ParsedHex> for NormalizedHex {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedHex) -> Result<Self, Self::Error> {
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            magic_type: s.magic_type,
            name: s.name,
            description: s.description,
            requires_confirmation: s.requires_confirmation,
            keywords: s.keywords,
            cast_invocation: s.cast_invocation.try_into()?,
            verify_invocation: s.verify_invocation.try_into()?,
        })
    }
}

impl TryFrom<ParsedInvocation> for NormalizedInvocation {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedInvocation) -> Result<Self, Self::Error> {
        Ok(Self {
            prefix_args: s.prefix_args,
            execution_command: s.execution_command,
            instrument_path: s.instrument_path,
            tool: s
                .tool
                .into_iter()
                .map(|t| t.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
