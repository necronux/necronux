// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    grimoire_validated_models::models::v0_4::{
        ValidatedRitual, ValidatedRitualCastStep, ValidatedRitualDispelStep,
        ValidatedRitualLayStep, ValidatedRitualStep,
    },
    unified_grimoire_bundle_model::{
        model::{
            UnifiedRitual, UnifiedRitualCastStep, UnifiedRitualDispelStep, UnifiedRitualLayStep,
            UnifiedRitualStep,
        },
        SchemaField,
    },
};

impl From<ValidatedRitual> for UnifiedRitual {
    fn from(s: ValidatedRitual) -> Self {
        Self {
            name: SchemaField::Present(s.name),
            description: SchemaField::Present(s.description),
            requires_confirmation: s.requires_confirmation.unwrap_or(false),
            keywords: SchemaField::Present(s.keywords.unwrap_or_default()),
            steps: SchemaField::Present(s.steps.into_iter().map(|s| s.into()).collect()),
        }
    }
}

impl From<ValidatedRitualStep> for UnifiedRitualStep {
    fn from(s: ValidatedRitualStep) -> Self {
        match s {
            ValidatedRitualStep::Cast(c) => UnifiedRitualStep::Cast(c.into()),
            ValidatedRitualStep::Lay(l) => UnifiedRitualStep::Lay(l.into()),
            ValidatedRitualStep::Dispel(d) => UnifiedRitualStep::Dispel(d.into()),
        }
    }
}

impl From<ValidatedRitualCastStep> for UnifiedRitualCastStep {
    fn from(s: ValidatedRitualCastStep) -> Self {
        Self {
            cast: SchemaField::Present(s.cast.into()),
            requires_confirmation: s.requires_confirmation.unwrap_or(false),
            auto_affirm: s.auto_affirm.unwrap_or(false),
        }
    }
}

impl From<ValidatedRitualDispelStep> for UnifiedRitualDispelStep {
    fn from(s: ValidatedRitualDispelStep) -> Self {
        Self {
            dispel: SchemaField::Present(s.dispel.into()),
            requires_confirmation: s.requires_confirmation.unwrap_or(false),
        }
    }
}

impl From<ValidatedRitualLayStep> for UnifiedRitualLayStep {
    fn from(s: ValidatedRitualLayStep) -> Self {
        Self {
            lay: SchemaField::Present(s.lay.into()),
            requires_confirmation: s.requires_confirmation.unwrap_or(false),
            auto_discern: s.auto_discern.unwrap_or(false),
        }
    }
}
