// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

pub mod common_metadata;
#[cfg(feature = "grimoire_schema_v0")]
mod v0;

#[cfg(feature = "grimoire_schema_v0")]
pub use v0::*;

// RFBs are Required Field Barriers for serde to not let serde throw its own unhelpful error
// when a required field is missing. Instead a new helpful error is thrown for the same in
// normalization. Instead of a new enum, Option is used when parsing to act as a RFB. This is
// because RFBs work as intended only with Option due to how serde sees Option.
