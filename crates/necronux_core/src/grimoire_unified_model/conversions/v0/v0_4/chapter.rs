// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    grimoire_unified_model::{SchemaField, model::UnifiedChapter},
    models::v0_4::ValidatedChapter,
};
use std::collections::HashMap;

impl From<ValidatedChapter> for UnifiedChapter {
    fn from(s: ValidatedChapter) -> Self {
        Self {
            grimoire_metadata: SchemaField::Present(s.grimoire_metadata.into()),
            name: SchemaField::Present(s.name),
            description: SchemaField::Present(s.description),
            spells: SchemaField::Present(s.spells.map(|map| {
                map.into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect::<HashMap<_, _>>()
            })),
            hexes: SchemaField::Present(s.hexes.map(|map| {
                map.into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect::<HashMap<_, _>>()
            })),
            requires_confirmation: SchemaField::Present(s.requires_confirmation),
        }
    }
}
