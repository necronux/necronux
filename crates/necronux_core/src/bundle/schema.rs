// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Bundle {
    config: BundleConfig,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct BundleConfig {
    loglevel: String,
}
