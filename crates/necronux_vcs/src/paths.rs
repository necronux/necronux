// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use anyhow::Context;

pub fn active_clone_path() -> anyhow::Result<std::path::PathBuf> {
    let data_local_dir =
        necronux_utils::paths::data_local_path().context("Failed to get active clone path")?;
    Ok(data_local_dir.join("active_clone"))
}

pub fn backup_active_clone_path() -> anyhow::Result<std::path::PathBuf> {
    let data_local_dir = necronux_utils::paths::data_local_path()
        .context("Failed to get active clone backup path")?;
    Ok(data_local_dir.join("backup_active_clone"))
}
