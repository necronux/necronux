// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::ValidateGrimoireError,
    grimoire_normalized_models::models::v0_4::{
        NormalizedRitual, NormalizedRitualCastStep, NormalizedRitualDispelStep,
        NormalizedRitualStep, NormalizedSpellOrHex,
    },
    models::v0_4::{
        ValidatedRitual, ValidatedRitualCastStep, ValidatedRitualDispelStep, ValidatedRitualStep,
        ValidatedSpellOrHex,
    },
    validators,
};

impl TryFrom<NormalizedRitual> for ValidatedRitual {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedRitual) -> Result<Self, Self::Error> {
        let parent_object = "Ritual";
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            ritual_type: validators::ensure_str_is_not_empty(
                s.ritual_type,
                "ritualType",
                parent_object,
            )?,
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
            steps: validators::ensure_vec_is_not_empty(s.steps, "steps", parent_object)?
                .into_iter()
                .map(|s| s.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<NormalizedRitualStep> for ValidatedRitualStep {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedRitualStep) -> Result<Self, Self::Error> {
        match s {
            NormalizedRitualStep::Cast(c) => Ok(ValidatedRitualStep::Cast(c.try_into()?)),
            NormalizedRitualStep::Dispel(d) => Ok(ValidatedRitualStep::Dispel(d.try_into()?)),
        }
    }
}

impl TryFrom<NormalizedRitualCastStep> for ValidatedRitualCastStep {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedRitualCastStep) -> Result<Self, Self::Error> {
        Ok(Self {
            cast: s.cast.try_into()?,
            requires_confirmation: s.requires_confirmation,
            auto_verify: s.auto_verify,
        })
    }
}

impl TryFrom<NormalizedRitualDispelStep> for ValidatedRitualDispelStep {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedRitualDispelStep) -> Result<Self, Self::Error> {
        Ok(Self {
            dispel: s.dispel.try_into()?,
            requires_confirmation: s.requires_confirmation,
        })
    }
}

impl TryFrom<NormalizedSpellOrHex> for ValidatedSpellOrHex {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedSpellOrHex) -> Result<Self, Self::Error> {
        match s {
            NormalizedSpellOrHex::Spell(s) => Ok(ValidatedSpellOrHex::Spell(s.try_into()?)),
            NormalizedSpellOrHex::Hex(h) => Ok(ValidatedSpellOrHex::Hex(h.try_into()?)),
        }
    }
}
