// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Bundle {
    config: BundleConfig,
}

#[derive(Deserialize)]
struct BundleConfig {
    loglevel: String,
}
