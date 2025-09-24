// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::schemas::common_metadata;
#[cfg(feature = "grimoire_schema_v0")]
use super::schemas::v0_4;

#[derive(Debug)]
pub enum ParsedGrimoire {
    CommonMetadata(common_metadata::ParsedGrimoire),

    #[cfg(feature = "grimoire_schema_v0")]
    V0_4(Box<v0_4::ParsedGrimoire>),
}
