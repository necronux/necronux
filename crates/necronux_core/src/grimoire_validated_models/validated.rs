// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::models::common_metadata;
#[cfg(feature = "grimoire_schema_v0")]
use super::models::v0_4;

#[derive(Debug)]
pub enum ValidatedGrimoire {
    CommonMetadata(common_metadata::ValidatedGrimoire),

    #[cfg(feature = "grimoire_schema_v0")]
    V0_4(Box<v0_4::ValidatedGrimoire>),
}
