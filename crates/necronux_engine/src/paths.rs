// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use anyhow::Context;

pub fn grimoire_root_path() -> anyhow::Result<std::path::PathBuf> {
    let data_local_dir =
        necronux_utils::paths::data_local_path().context("Failed to get necronux grimoire path")?;
    Ok(data_local_dir
        .join("active_clone")
        .join("necronux.grimoire"))
}

pub fn grimoire_subdir_path() -> anyhow::Result<std::path::PathBuf> {
    let data_local_dir =
        necronux_utils::paths::data_local_path().context("Failed to get necronux grimoire path")?;
    Ok(data_local_dir
        .join("active_clone")
        .join("necronux")
        .join("necronux.grimoire"))
}

pub fn locate_grimoire_path() -> anyhow::Result<std::path::PathBuf> {
    let root_path = grimoire_root_path()?;
    if necronux_utils::paths::path_exists(&root_path, "necronux.grimoire")? {
        return Ok(root_path);
    }

    let subdir_path = grimoire_subdir_path()?;
    if necronux_utils::paths::path_exists(&subdir_path, "necronux.grimoire")? {
        return Ok(subdir_path);
    }

    Err(anyhow::anyhow!(
        "necronux.grimoire file not found in '{}' or '{}'",
        root_path.display(),
        subdir_path.display()
    ))
}
