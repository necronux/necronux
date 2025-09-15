// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::{FsError, FsTarget};
use necronux_macros::trace_instrument;
use std::{path::Path, result as stdrt};
use tracing::debug;

#[trace_instrument(level = "debug", fields(label = %label, path = %path.display()))]
pub fn remove_dir_all(path: &Path, label: &str) -> stdrt::Result<(), FsError> {
    debug!(
        label = %label,
        path = %path.display(),
        "Attempting to delete directory...",
    );

    match std::fs::remove_dir_all(path) {
        Ok(_) => {
            debug!(
                label = %label,
                path = %path.display(),
                "Successfully deleted directory",
            );
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            debug!(
                label = %label,
                path = %path.display(),
                "No deletion needed as directory does not exist",
            );
            Ok(())
        }
        Err(e) => Err(FsError::RemoveError {
            label: label.to_string(),
            path: path.to_path_buf(),
            target: FsTarget::Directory,
            source: e,
        }),
    }
}
