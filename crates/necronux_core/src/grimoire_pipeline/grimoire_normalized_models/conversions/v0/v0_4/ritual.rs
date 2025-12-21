// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError,
    grimoire_normalized_models::models::v0_4::{
        NormalizedRitual, NormalizedRitualCastStep, NormalizedRitualDispelStep,
        NormalizedRitualLayStep, NormalizedRitualStep,
    },
    grimoire_parsed_models::models::v0_4::{
        ParsedRitual, ParsedRitualCastStep, ParsedRitualDispelStep, ParsedRitualLayStep,
        ParsedRitualStep,
    },
    grimoire_pipeline::normalizers,
};

impl TryFrom<ParsedRitual> for NormalizedRitual {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedRitual) -> Result<Self, Self::Error> {
        let parent_object = "Ritual";
        Ok(Self {
            name: normalizers::ensure_req_field_is_not_missing(s.name, "name", parent_object)?,
            description: s.description,
            requires_confirmation: s.requires_confirmation,
            keywords: s.keywords,
            steps: normalizers::ensure_req_field_is_not_missing(s.steps, "steps", parent_object)?
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
            ParsedRitualStep::Lay(l) => Ok(NormalizedRitualStep::Lay(l.try_into()?)),
            ParsedRitualStep::Dispel(d) => Ok(NormalizedRitualStep::Dispel(d.try_into()?)),
        }
    }
}

impl TryFrom<ParsedRitualCastStep> for NormalizedRitualCastStep {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedRitualCastStep) -> Result<Self, Self::Error> {
        let parent_object = "RitualCastStep";
        Ok(Self {
            cast: normalizers::ensure_req_field_is_not_missing(s.cast, "cast", parent_object)?
                .try_into()?,
            requires_confirmation: s.requires_confirmation,
            auto_affirm: s.auto_affirm,
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
                parent_object,
            )?
            .try_into()?,
            requires_confirmation: s.requires_confirmation,
        })
    }
}

impl TryFrom<ParsedRitualLayStep> for NormalizedRitualLayStep {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedRitualLayStep) -> Result<Self, Self::Error> {
        let parent_object = "RitualLayStep";
        Ok(Self {
            lay: normalizers::ensure_req_field_is_not_missing(s.lay, "lay", parent_object)?
                .try_into()?,
            requires_confirmation: s.requires_confirmation,
            auto_discern: s.auto_discern,
        })
    }
}
