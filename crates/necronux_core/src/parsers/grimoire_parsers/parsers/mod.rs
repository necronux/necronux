// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

mod common_metadata;
#[cfg(feature = "grimoire_schema_v0")]
mod v0;

pub use common_metadata::*;
#[cfg(feature = "grimoire_schema_v0")]
pub use v0::*;
