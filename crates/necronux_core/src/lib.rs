// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

mod bundle;
mod error_reporter;
mod logger;

pub use bundle::loader::load_bundle_file;
pub use error_reporter::init_error_reporter;
pub use logger::init_logger;
