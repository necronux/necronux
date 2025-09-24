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
};

impl TryFrom<NormalizedSpell> for ValidatedSpell {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedSpell) -> Result<Self, Self::Error> {
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

impl TryFrom<NormalizedHex> for ValidatedHex {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedHex) -> Result<Self, Self::Error> {
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

impl TryFrom<NormalizedInvocation> for ValidatedInvocation {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedInvocation) -> Result<Self, Self::Error> {
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
