// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::UnifiedGrimoire;
use crate::grimoire_schemas::minimal_metadata as mm;

impl From<mm::Grimoire> for UnifiedGrimoire {
    fn from(s: mm::Grimoire) -> Self {
        Self {
            std_schema_version: Some(s.std_schema_version.to_string()),
            grimoire_metadata: None,
        }
    }
}
