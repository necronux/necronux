// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::Result;
use crate::error::FsError;
use std::{fs::File, path::Path};
use tracing::debug;

pub fn create_dir_all(path: &Path, label: &str) -> Result<()> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("create_dir_all", label = label).entered();

    std::fs::create_dir_all(path).map_err(|e| FsError::CreateDirError {
        label: label.to_string(),
        path: path.to_path_buf(),
        source: e,
    })?;
    debug!(
        "Successfully created {label} directory at '{}'",
        path.display()
    );
    Ok(())
}

pub fn create_file(path: &Path, label: &str) -> Result<File> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("create_file", label = label).entered();

    let file = std::fs::File::create(path).map_err(|e| FsError::CreateFileError {
        label: label.to_string(),
        path: path.to_path_buf(),
        source: e,
    })?;
    debug!("Successfully created {label} file at '{}'", path.display());
    Ok(file)
}
