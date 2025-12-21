// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    grimoire_validated_models::models::v0_4::ValidatedChapter,
    unified_grimoire_bundle_model::{model::UnifiedChapter, SchemaField},
};
use std::collections::HashMap;

impl From<ValidatedChapter> for UnifiedChapter {
    fn from(s: ValidatedChapter) -> Self {
        Self {
            name: SchemaField::Present(s.name),
            description: SchemaField::Present(s.description),
            spells: SchemaField::Present(
                s.spells
                    .map(|s| {
                        s.into_iter()
                            .map(|(k, v)| (k, v.into()))
                            .collect::<HashMap<_, _>>()
                    })
                    .unwrap_or_default(),
            ),
            hexes: SchemaField::Present(
                s.hexes
                    .map(|h| {
                        h.into_iter()
                            .map(|(k, v)| (k, v.into()))
                            .collect::<HashMap<_, _>>()
                    })
                    .unwrap_or_default(),
            ),
            requires_confirmation: s.requires_confirmation.unwrap_or(false),
        }
    }
}
