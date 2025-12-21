// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

#[cfg(feature = "grimoire_schema_v0")]
mod v0;

#[cfg(feature = "grimoire_schema_v0")]
pub use v0::*;
