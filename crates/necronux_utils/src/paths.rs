// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::PathError;
use directories::ProjectDirs;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, PathError>;

pub fn data_local_necronux_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("", "necronux", "necronux")
        .ok_or(PathError::GetDataLocalNecronuxDirError)?;
    Ok(proj_dirs.data_local_dir().to_path_buf())
}

pub fn current_grimoire_path() -> Result<PathBuf> {
    Ok(data_local_necronux_path()?.join("current_grimoire"))
}
