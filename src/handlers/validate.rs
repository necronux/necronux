// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use anyhow::Context;

pub struct ValidateHandler;

impl ValidateHandler {
    pub fn handle_validate() -> anyhow::Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::info_span!("handle_validate").entered();

        #[cfg(feature = "stdschema_v1")]
        let _pkl = necronux_core::engine::stdschema_v1::parse_grimoire_file()
            .context("Failed to validate necronux.grimoire file")?;

        Ok(())
    }
}
