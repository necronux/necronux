// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::Result;
use crate::error::FsError;
use std::path::Path;
use tracing::debug;

pub fn rename_dir_if_exists(old_path: &Path, old_path_label: &str, new_path: &Path) -> Result<()> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("rename_dir", old_path_label = old_path_label).entered();

    if !super::path_exists(old_path, old_path_label)? {
        return Err(FsError::RenameDirError {
            old_path_label: old_path_label.to_string(),
            old_path: old_path.to_path_buf(),
            new_path: new_path.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Path does not exist when attempting to rename",
            ),
        });
    }

    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("rename_dir_op", old_path_label = old_path_label).entered();
    std::fs::rename(old_path, new_path).map_err(|e| FsError::RenameDirError {
        old_path_label: old_path_label.to_string(),
        old_path: old_path.to_path_buf(),
        new_path: new_path.to_path_buf(),
        source: e,
    })?;
    debug!(
        "Successfully renamed {old_path_label} at '{}' to '{}'",
        old_path.display(),
        new_path.display()
    );
    Ok(())
}

pub fn copy_if_exists(from_path: &Path, from_path_label: &str, to_path: &Path) -> Result<()> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("copy", from_path_label = from_path_label).entered();

    if !super::path_exists(from_path, from_path_label)? {
        return Err(FsError::CopyFileError {
            from_path_label: from_path_label.to_string(),
            from_path: from_path.to_path_buf(),
            to_path: to_path.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Path does not exist when attempting to copy",
            ),
        });
    }

    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("copy_op", from_path_label = from_path_label).entered();
    std::fs::copy(from_path, to_path).map_err(|e| FsError::CopyFileError {
        from_path_label: from_path_label.to_string(),
        from_path: from_path.to_path_buf(),
        to_path: to_path.to_path_buf(),
        source: e,
    })?;
    debug!(
        "Successfully copied from {from_path_label} at '{}' to '{}'",
        from_path.display(),
        to_path.display()
    );
    Ok(())
}
