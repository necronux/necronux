// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{UnifiedCommonMetadata, UnifiedGrimoire, UnifiedPackageMetadata};

#[derive(Debug)]
pub struct UnifiedGrimoireBundle {
    pub package_metadata: UnifiedPackageMetadata,
    pub common_metadata: UnifiedCommonMetadata,
    pub grimoire: UnifiedGrimoire,
}
