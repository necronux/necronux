// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::{FsError, FsTarget};
use necronux_macros::trace_instrument;
use std::{fs::File, path::Path, result as stdrt};
use tracing::debug;

#[trace_instrument(level = "debug", fields(label = %label, path = %path.display()))]
pub fn create_dir_all(path: &Path, label: &str) -> stdrt::Result<(), FsError> {
    debug!(
        label = %label,
        path = %path.display(),
        "Attempting to create directory...",
    );

    std::fs::create_dir_all(path).map_err(|e| FsError::CreateError {
        label: label.to_string(),
        path: path.to_path_buf(),
        target: FsTarget::Directory,
        source: e,
    })?;

    debug!(
        label = %label,
        path = %path.display(),
        "Successfully created directory",
    );
    Ok(())
}

#[trace_instrument(level = "debug", fields(label = %label, path = %path.display()))]
pub fn create_file(path: &Path, label: &str) -> stdrt::Result<File, FsError> {
    debug!(
        label = %label,
        path = %path.display(),
        "Attempting to create file..."
    );

    let file = std::fs::File::create(path).map_err(|e| FsError::CreateError {
        label: label.to_string(),
        path: path.to_path_buf(),
        target: FsTarget::File,
        source: e,
    })?;

    debug!(
        label = %label,
        path = %path.display(),
        "Successfully created file"
    );
    Ok(file)
}
