// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use anyhow::anyhow;
use directories::ProjectDirs;
use std::path::PathBuf;
use tracing::debug;

pub fn data_local_path() -> anyhow::Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("", "necronux", "necronux")
        .ok_or_else(|| anyhow!("Could not determine data local directory"))?;
    Ok(proj_dirs.data_local_dir().to_path_buf())
}

pub fn path_exists(path: &std::path::Path, label: &str) -> anyhow::Result<bool> {
    match path.try_exists() {
        Ok(true) => {
            debug!("{} exists at '{}'", label, path.display());
            Ok(true)
        }
        Ok(false) => {
            debug!("{} does not exist at '{}'", label, path.display());
            Ok(false)
        }
        Err(err) => Err(anyhow!(
            "{} existence check failed at '{}': {}",
            label,
            path.display(),
            err
        )),
    }
}
