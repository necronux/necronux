// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

pub mod fs;
#[cfg(feature = "git")]
mod git;
pub mod paths;
pub mod status;
pub mod validate;

#[cfg(feature = "git")]
pub use git::*;
