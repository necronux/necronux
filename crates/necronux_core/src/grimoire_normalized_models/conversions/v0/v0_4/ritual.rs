// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError,
    grimoire_normalized_models::{
        models::v0_4::{
            NormalizedRitual, NormalizedRitualCastStep, NormalizedRitualDispelStep,
            NormalizedRitualStep, NormalizedSpellOrHex,
        },
        normalizers,
    },
    grimoire_schemas::schemas::v0_4::{
        ParsedRitual, ParsedRitualCastStep, ParsedRitualDispelStep, ParsedRitualStep,
        ParsedSpellOrHex,
    },
};

impl TryFrom<ParsedRitual> for NormalizedRitual {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedRitual) -> Result<Self, Self::Error> {
        let parent_object = "Ritual";
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            ritual_type: normalizers::ensure_req_field_is_not_missing(
                s.ritual_type,
                "ritualType",
                &parent_object,
            )?,
            name: normalizers::ensure_req_field_is_not_missing(s.name, "name", &parent_object)?,
            description: s.description,
            requires_confirmation: normalizers::ensure_req_field_is_not_missing(
                s.requires_confirmation,
                "requiresConfirmation",
                &parent_object,
            )?,
            keywords: s.keywords,
            steps: normalizers::ensure_req_field_is_not_missing(s.steps, "steps", &parent_object)?
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
        let parent_object = "RitualCastStep";
        Ok(Self {
            cast: normalizers::ensure_req_field_is_not_missing(s.cast, "cast", &parent_object)?
                .try_into()?,
            requires_confirmation: normalizers::ensure_req_field_is_not_missing(
                s.requires_confirmation,
                "requiresConfirmation",
                &parent_object,
            )?,
            auto_verify: normalizers::ensure_req_field_is_not_missing(
                s.auto_verify,
                "autoVerify",
                &parent_object,
            )?,
        })
    }
}

impl TryFrom<ParsedRitualDispelStep> for NormalizedRitualDispelStep {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedRitualDispelStep) -> Result<Self, Self::Error> {
        let parent_object = "RitualDispelStep";
        Ok(Self {
            dispel: normalizers::ensure_req_field_is_not_missing(
                s.dispel,
                "dispel",
                &parent_object,
            )?
            .try_into()?,
            requires_confirmation: normalizers::ensure_req_field_is_not_missing(
                s.requires_confirmation,
                "requiresConfirmation",
                &parent_object,
            )?,
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
