// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError,
    grimoire_normalized_models::models::v0::{
        NormalizedRitual, NormalizedRitualCastStep, NormalizedRitualDispelStep,
        NormalizedRitualStep, NormalizedSpellOrHex,
    },
    grimoire_schemas::schemas::v0::{
        ParsedRitual, ParsedRitualCastStep, ParsedRitualDispelStep, ParsedRitualStep,
        ParsedSpellOrHex,
    },
};

impl TryFrom<ParsedRitual> for NormalizedRitual {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedRitual) -> Result<Self, Self::Error> {
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            ritual_type: s.ritual_type,
            name: s.name,
            description: s.description,
            requires_confirmation: s.requires_confirmation,
            keywords: s.keywords,
            steps: s
                .steps
                .into_iter()
                .map(|s| s.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<ParsedRitualStep> for NormalizedRitualStep {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedRitualStep) -> Result<Self, Self::Error> {
        match s {
            ParsedRitualStep::Cast(c) => Ok(NormalizedRitualStep::Cast(c.try_into()?)),
            ParsedRitualStep::Dispel(d) => Ok(NormalizedRitualStep::Dispel(d.try_into()?)),
        }
    }
}

impl TryFrom<ParsedRitualCastStep> for NormalizedRitualCastStep {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedRitualCastStep) -> Result<Self, Self::Error> {
        Ok(Self {
            cast: s.cast.try_into()?,
            requires_confirmation: s.requires_confirmation,
            auto_verify: s.auto_verify,
        })
    }
}

impl TryFrom<ParsedRitualDispelStep> for NormalizedRitualDispelStep {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedRitualDispelStep) -> Result<Self, Self::Error> {
        Ok(Self {
            dispel: s.dispel.try_into()?,
            requires_confirmation: s.requires_confirmation,
        })
    }
}

impl TryFrom<ParsedSpellOrHex> for NormalizedSpellOrHex {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedSpellOrHex) -> Result<Self, Self::Error> {
        match s {
            ParsedSpellOrHex::Spell(s) => Ok(NormalizedSpellOrHex::Spell(s.try_into()?)),
            ParsedSpellOrHex::Hex(h) => Ok(NormalizedSpellOrHex::Hex(h.try_into()?)),
        }
    }
}
