// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    grimoire_validated_models::models::v0_4::{
        ValidatedCoreContents, ValidatedGrimoire, ValidatedGrimoireMetadata,
    },
    unified_grimoire_bundle_model::{
        model::{UnifiedCoreContents, UnifiedGrimoire, UnifiedGrimoireMetadata},
        SchemaField,
    },
};
use std::collections::HashMap;

impl From<ValidatedGrimoire> for UnifiedGrimoire {
    fn from(s: ValidatedGrimoire) -> Self {
        Self {
            grimoire_metadata: SchemaField::Present(s.grimoire_metadata.map(|gm| gm.into())),
            core_contents: SchemaField::Present(s.core_contents.into()),
        }
    }
}

impl From<ValidatedGrimoireMetadata> for UnifiedGrimoireMetadata {
    fn from(s: ValidatedGrimoireMetadata) -> Self {
        Self {
            grimoire_keywords: SchemaField::Present(s.grimoire_keywords.unwrap_or_default()),
            grimoire_additional_metadata: SchemaField::Present(
                s.grimoire_additional_metadata.unwrap_or_default(),
            ),
        }
    }
}

impl From<ValidatedCoreContents> for UnifiedCoreContents {
    fn from(s: ValidatedCoreContents) -> Self {
        Self {
            chapters: SchemaField::Present(
                s.chapters
                    .map(|c| {
                        c.into_iter()
                            .map(|(k, v)| (k, v.into()))
                            .collect::<HashMap<_, _>>()
                    })
                    .unwrap_or_default(),
            ),
            rituals: SchemaField::Present(
                s.rituals
                    .map(|r| {
                        r.into_iter()
                            .map(|(k, v)| (k, v.into()))
                            .collect::<HashMap<_, _>>()
                    })
                    .unwrap_or_default(),
            ),
            auto_perform_rituals: SchemaField::Present(s.auto_perform_rituals.unwrap_or_default()),
            requires_confirmation: s.requires_confirmation.unwrap_or(false),
        }
    }
}
