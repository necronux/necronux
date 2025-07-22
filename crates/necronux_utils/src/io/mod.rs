// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

mod transfer;

pub use transfer::*;

use crate::error::IoError;

pub type Result<T> = std::result::Result<T, IoError>;
