// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

mod kind;
#[macro_use]
mod macros;
mod msg;
mod msg_body;
mod msg_prefix;
mod msg_prefix_with_spinner_ticks;
mod msg_spinner;

pub use kind::*;
pub use msg_body::*;
