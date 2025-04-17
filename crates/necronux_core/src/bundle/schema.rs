// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Bundle {
    pub bundle: BundleClass,
}

#[derive(Deserialize)]
pub struct BundleClass {
    pub name: String,
    pub description: String,
    pub apps: Vec<String>,
}
