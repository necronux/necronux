// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::{FsError, FsTarget};
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
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(FsError::RenameError {
            old_path_label: old_path_label.to_string(),
            old_path: old_path.to_path_buf(),
            new_path: new_path.to_path_buf(),
            target: FsTarget::Directory,
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Path does not exist when attempting to rename",
            ),
        }),
        Err(e) => Err(FsError::RenameError {
            old_path_label: old_path_label.to_string(),
            old_path: old_path.to_path_buf(),
            new_path: new_path.to_path_buf(),
            target: FsTarget::Directory,
            source: e,
        }),
    }
}

#[trace_instrument(level = "debug", fields(from_path_label = %from_path_label, from_path = %from_path.display(), to_path = %to_path.display()))]
pub fn copy(from_path: &Path, from_path_label: &str, to_path: &Path) -> stdrt::Result<(), FsError> {
    let (content_type, target) = if from_path_label.to_lowercase().contains("file") {
        ("file", FsTarget::File)
    } else if from_path_label.to_lowercase().contains("directory") {
        ("directory", FsTarget::Directory)
    } else {
        // Fallback to file
        ("file", FsTarget::File)
    };

    debug!(
        from_path_label = %from_path_label,
        from_path = %from_path.display(),
        to_path = %to_path.display(),
        content_type = %content_type,
        "Attempting to copy {content_type}...",
    );

    match std::fs::copy(from_path, to_path) {
        Ok(_) => {
            debug!(
                from_path = %from_path.display(),
                to_path = %to_path.display(),
                content_type = %content_type,
                "Successfully copied {content_type}",
            );
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(FsError::CopyError {
            from_path_label: from_path_label.to_string(),
            from_path: from_path.to_path_buf(),
            to_path: to_path.to_path_buf(),
            target,
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Path does not exist when attempting to copy",
            ),
        }),
        Err(e) => Err(FsError::CopyError {
            from_path_label: from_path_label.to_string(),
            from_path: from_path.to_path_buf(),
            to_path: to_path.to_path_buf(),
            target,
            source: e,
        }),
    }
}
