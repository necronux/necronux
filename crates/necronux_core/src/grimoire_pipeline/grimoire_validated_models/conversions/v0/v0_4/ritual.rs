// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::ValidateGrimoireError,
    grimoire_normalized_models::models::v0_4::{
        NormalizedRitual, NormalizedRitualCastStep, NormalizedRitualDispelStep,
        NormalizedRitualLayStep, NormalizedRitualStep,
    },
    grimoire_pipeline::validators,
    grimoire_validated_models::models::v0_4::{
        ValidatedRitual, ValidatedRitualCastStep, ValidatedRitualDispelStep,
        ValidatedRitualLayStep, ValidatedRitualStep,
    },
};

impl TryFrom<NormalizedRitual> for ValidatedRitual {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedRitual) -> Result<Self, Self::Error> {
        let parent_object = "Ritual";
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
            NormalizedRitualStep::Lay(l) => Ok(ValidatedRitualStep::Lay(l.try_into()?)),
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
            auto_affirm: s.auto_affirm,
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

impl TryFrom<NormalizedRitualLayStep> for ValidatedRitualLayStep {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedRitualLayStep) -> Result<Self, Self::Error> {
        Ok(Self {
            lay: s.lay.try_into()?,
            requires_confirmation: s.requires_confirmation,
            auto_discern: s.auto_discern,
        })
    }
}
