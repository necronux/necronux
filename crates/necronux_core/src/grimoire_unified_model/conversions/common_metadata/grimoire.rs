// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    grimoire_unified_model::{SchemaField, model::UnifiedGrimoire},
    models::common_metadata::ValidatedGrimoire,
};

impl From<ValidatedGrimoire> for UnifiedGrimoire {
    fn from(s: ValidatedGrimoire) -> Self {
        Self {
            schema_version_info: SchemaField::Present(s.schema_version_info.into()),
            grimoire_metadata: SchemaField::Absent,
            core_contents: SchemaField::Absent,
        }
    }
}
