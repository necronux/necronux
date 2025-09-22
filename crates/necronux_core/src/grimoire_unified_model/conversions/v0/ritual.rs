// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    grimoire_unified_model::{
        SchemaField,
        model::{
            UnifiedRitual, UnifiedRitualCastStep, UnifiedRitualDispelStep, UnifiedRitualStep,
            UnifiedSpellOrHex,
        },
    },
    models::v0::{
        ValidatedRitual, ValidatedRitualCastStep, ValidatedRitualDispelStep, ValidatedRitualStep,
        ValidatedSpellOrHex,
    },
};

impl From<ValidatedRitual> for UnifiedRitual {
    fn from(s: ValidatedRitual) -> Self {
        Self {
            grimoire_metadata: SchemaField::Present(s.grimoire_metadata.into()),
            ritual_type: SchemaField::Present(s.ritual_type),
            name: SchemaField::Present(s.name),
            description: SchemaField::Present(s.description),
            requires_confirmation: SchemaField::Present(s.requires_confirmation),
            keywords: SchemaField::Present(s.keywords),
            steps: SchemaField::Present(s.steps.into_iter().map(|s| s.into()).collect()),
        }
    }
}

impl From<ValidatedRitualStep> for UnifiedRitualStep {
    fn from(s: ValidatedRitualStep) -> Self {
        match s {
            ValidatedRitualStep::Cast(c) => UnifiedRitualStep::Cast(c.into()),
            ValidatedRitualStep::Dispel(d) => UnifiedRitualStep::Dispel(d.into()),
        }
    }
}

impl From<ValidatedRitualCastStep> for UnifiedRitualCastStep {
    fn from(s: ValidatedRitualCastStep) -> Self {
        Self {
            cast: SchemaField::Present(s.cast.into()),
            requires_confirmation: SchemaField::Present(s.requires_confirmation),
            auto_verify: SchemaField::Present(s.auto_verify),
        }
    }
}

impl From<ValidatedRitualDispelStep> for UnifiedRitualDispelStep {
    fn from(s: ValidatedRitualDispelStep) -> Self {
        Self {
            dispel: SchemaField::Present(s.dispel.into()),
            requires_confirmation: SchemaField::Present(s.requires_confirmation),
        }
    }
}

impl From<ValidatedSpellOrHex> for UnifiedSpellOrHex {
    fn from(s: ValidatedSpellOrHex) -> Self {
        match s {
            ValidatedSpellOrHex::Spell(s) => UnifiedSpellOrHex::Spell(s.into()),
            ValidatedSpellOrHex::Hex(h) => UnifiedSpellOrHex::Hex(h.into()),
        }
    }
}
