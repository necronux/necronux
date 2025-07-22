// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Enchantments {
    #[serde(rename = "installAllRequiredToolsOnConnect")]
    pub install_all_required_tools_on_connect: bool,

    #[serde(rename = "autoVerifySpellsAndHexesAfterCasting")]
    pub auto_verify_spells_and_hexes_after_casting: bool,
}
