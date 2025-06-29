// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

mod paths;
#[cfg(feature = "stdschema_v1")]
pub mod stdschema_v1;

pub use paths::*;
