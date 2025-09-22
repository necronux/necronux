// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

pub mod common_metadata;
#[cfg(feature = "grimoire_schema_v0")]
pub mod v0;

// SLRFBs are Second Level Required Field Barriers for serde to not let first level required field
// fallback to default when any of its second level required field is missing. These barriers
// cover deeper levels too. Option is used before normalization to act as a SLRFB.
