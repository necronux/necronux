// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::UnifiedGrimoire;

impl From<crate::grimoire_schemas::minimal_metadata::Grimoire> for UnifiedGrimoire {
    fn from(mm: crate::grimoire_schemas::minimal_metadata::Grimoire) -> Self {
        Self {
            std_schema_version: Some(mm.std_schema_version.to_string()),
            grimoire_metadata: None,
        }
    }
}
