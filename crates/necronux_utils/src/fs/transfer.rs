// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::FsError;
use necronux_macros::trace_instrument;
use std::{path::Path, result as stdrt};
use tracing::debug;

#[trace_instrument(level = "debug", fields(old_path_label = %old_path_label, old_path = %old_path.display(), new_path = %new_path.display()))]
pub fn rename_dir(
    old_path: &Path,
    old_path_label: &str,
    new_path: &Path,
) -> stdrt::Result<(), FsError> {
    debug!(
        old_path_label = %old_path_label,
        old_path = %old_path.display(),
        new_path = %new_path.display(),
        "Attempting to rename directory...",
    );

    match std::fs::rename(old_path, new_path) {
        Ok(_) => {
            debug!(
                old_path = %old_path.display(),
                new_path = %new_path.display(),
                "Successfully renamed directory",
            );
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(FsError::RenameDirError {
            old_path_label: old_path_label.to_string(),
            old_path: old_path.to_path_buf(),
            new_path: new_path.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Path does not exist when attempting to rename directory",
            ),
        }),
        Err(e) => Err(FsError::RenameDirError {
            old_path_label: old_path_label.to_string(),
            old_path: old_path.to_path_buf(),
            new_path: new_path.to_path_buf(),
            source: e,
        }),
    }
}

#[trace_instrument(level = "debug", fields(from_path_label = %from_path_label, from_path = %from_path.display(), to_path = %to_path.display()))]
pub fn copy(from_path: &Path, from_path_label: &str, to_path: &Path) -> stdrt::Result<(), FsError> {
    debug!(
        from_path_label = %from_path_label,
        from_path = %from_path.display(),
        to_path = %to_path.display(),
        "Attempting to copy file...",
    );

    match std::fs::copy(from_path, to_path) {
        Ok(_) => {
            debug!(
                from_path = %from_path.display(),
                to_path = %to_path.display(),
                "Successfully copied file",
            );
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(FsError::CopyFileError {
            from_path_label: from_path_label.to_string(),
            from_path: from_path.to_path_buf(),
            to_path: to_path.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Path does not exist when attempting to copy file",
            ),
        }),
        Err(e) => Err(FsError::CopyFileError {
            from_path_label: from_path_label.to_string(),
            from_path: from_path.to_path_buf(),
            to_path: to_path.to_path_buf(),
            source: e,
        }),
    }
}
