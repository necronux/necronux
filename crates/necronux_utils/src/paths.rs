// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use anyhow::{Context, anyhow};
use directories::{BaseDirs, ProjectDirs};
use std::path::PathBuf;
use tracing::debug;

pub fn data_local_path() -> anyhow::Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("", "necronux", "necronux")
        .ok_or_else(|| anyhow!("Could not determine data local directory"))?;
    Ok(proj_dirs.data_local_dir().to_path_buf())
}

pub fn home_path() -> anyhow::Result<PathBuf> {
    let base_dirs = BaseDirs::new().ok_or_else(|| anyhow!("Could not determine home directory"))?;
    Ok(base_dirs.home_dir().to_path_buf())
}

pub fn ssh_path() -> anyhow::Result<std::path::PathBuf> {
    let home_dir = home_path().context("Failed to get .ssh directory path")?;
    Ok(home_dir.join(".ssh"))
}

pub fn ssh_private_key_path() -> anyhow::Result<PathBuf> {
    let ssh_dir = ssh_path().context("Failed to get ssh private key path")?;
    Ok(ssh_dir.join("id_rsa"))
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
