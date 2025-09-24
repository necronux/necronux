// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError,
    grimoire_normalized_models::models::v0_4::{
        NormalizedRitual, NormalizedRitualCastStep, NormalizedRitualDispelStep,
        NormalizedRitualStep, NormalizedSpellOrHex,
    },
    grimoire_schemas::schemas::v0_4::{
        ParsedRitual, ParsedRitualCastStep, ParsedRitualDispelStep, ParsedRitualStep,
        ParsedSpellOrHex,
    },
};

impl TryFrom<ParsedRitual> for NormalizedRitual {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedRitual) -> Result<Self, Self::Error> {
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            ritual_type: s.ritual_type.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "ritualType".to_string(),
                    parent_object_name: "Ritual".to_string(),
                }
            })?,
            name: s
                .name
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "name".to_string(),
                    parent_object_name: "Ritual".to_string(),
                })?,
            description: s.description,
            requires_confirmation: s.requires_confirmation.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "requiresConfirmation".to_string(),
                    parent_object_name: "Ritual".to_string(),
                }
            })?,
            keywords: s.keywords,
            steps: s
                .steps
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "steps".to_string(),
                    parent_object_name: "Ritual".to_string(),
                })?
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
            cast: s
                .cast
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "cast".to_string(),
                    parent_object_name: "RitualCastStep".to_string(),
                })?
                .try_into()?,
            requires_confirmation: s.requires_confirmation.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "requiresConfirmation".to_string(),
                    parent_object_name: "RitualCastStep".to_string(),
                }
            })?,
            auto_verify: s.auto_verify.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "autoVerify".to_string(),
                    parent_object_name: "RitualCastStep".to_string(),
                }
            })?,
        })
    }
}

impl TryFrom<ParsedRitualDispelStep> for NormalizedRitualDispelStep {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedRitualDispelStep) -> Result<Self, Self::Error> {
        Ok(Self {
            dispel: s
                .dispel
                .ok_or_else(|| NormalizeGrimoireError::MissingRequiredField {
                    field_name: "dispel".to_string(),
                    parent_object_name: "DispelCastStep".to_string(),
                })?
                .try_into()?,
            requires_confirmation: s.requires_confirmation.ok_or_else(|| {
                NormalizeGrimoireError::MissingRequiredField {
                    field_name: "requiresConfirmation".to_string(),
                    parent_object_name: "RitualDispelStep".to_string(),
                }
            })?,
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
