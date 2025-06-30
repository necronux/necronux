// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

pub struct ConnectHandler;

impl ConnectHandler {
    pub fn handle_connect() -> anyhow::Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::info_span!("handle_connect").entered();

        Ok(())
    }
}
