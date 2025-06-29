// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::stdschema::GrimoirePkl;
use anyhow::Context;

pub fn parse_grimoire_file() -> anyhow::Result<GrimoirePkl> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("parse_grimoire_file", stdschema_ver = 1).entered();

    let path =
        crate::paths::locate_grimoire_path().context("Failed to locate necronux grimoire path")?;
    let pkl: GrimoirePkl = rpkl::from_config(&path).with_context(|| {
        format!(
            "Failed to parse necronux.grimoire file at '{}'",
            path.display()
        )
    })?;

    Ok(pkl)
}
