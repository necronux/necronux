// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::PathError;
use directories::ProjectDirs;
use once_cell::sync::OnceCell;
use std::{path::PathBuf, result as stdrt};

static NECRONUX_PROJ_DIRS: OnceCell<ProjectDirs> = OnceCell::new();

pub fn necronux_proj_dir() -> stdrt::Result<&'static ProjectDirs, PathError> {
    NECRONUX_PROJ_DIRS.get_or_try_init(|| {
        ProjectDirs::from("", "necronux", "necronux").ok_or(PathError::NecronuxProjectDirError)
    })
}

pub fn data_local_necronux_path() -> stdrt::Result<PathBuf, PathError> {
    Ok(necronux_proj_dir()?.data_local_dir().to_path_buf())
}

pub fn current_grimoire_path() -> stdrt::Result<PathBuf, PathError> {
    Ok(data_local_necronux_path()?.join("current_grimoire"))
}

pub fn cache_necronux_path() -> stdrt::Result<PathBuf, PathError> {
    Ok(necronux_proj_dir()?.cache_dir().to_path_buf())
}

pub fn necronux_log_file_path() -> stdrt::Result<PathBuf, PathError> {
    Ok(cache_necronux_path()?.join("necronux_log.json"))
}
