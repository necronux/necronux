// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::FsError;
use necronux_macros::trace_instrument;
use std::{path::Path, result as stdrt};
use tracing::debug;

#[trace_instrument(level = "debug", fields(label = %label, path = %path.display()))]
pub fn path_exists(path: &Path, label: &str) -> stdrt::Result<bool, FsError> {
    debug!(
        label = %label,
        path = %path.display(),
        "Checking path existence..."
    );

    match path.try_exists() {
        Ok(true) => {
            debug!(
                label = %label,
                path = %path.display(),
                "Path exists"
            );
            Ok(true)
        }
        Ok(false) => {
            debug!(
                label = %label,
                path = %path.display(),
                "Path does not exist",
            );
            Ok(false)
        }
        Err(e) => Err(FsError::CheckPathExistsError {
            label: label.to_string(),
            path: path.to_path_buf(),
            source: e,
        }),
    }
}

#[trace_instrument(level = "debug", fields(label = %label, path = %path.display()))]
pub fn file_exists(path: &Path, label: &str) -> stdrt::Result<bool, FsError> {
    debug!(
        label = %label,
        path = %path.display(),
        "Checking if file exists..."
    );

    match std::fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_file() {
                debug!(
                    label = %label,
                    path = %path.display(),
                    "Path exists and is a valid file",
                );
                Ok(true)
            } else {
                debug!(
                    label = %label,
                    path = %path.display(),
                    "Path exists but is not a valid file",
                );
                Ok(false)
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            debug!(
                label = %label,
                path = %path.display(),
                "Path does not exist",
            );
            Ok(false)
        }
        Err(e) => Err(FsError::CheckPathExistsError {
            label: label.to_string(),
            path: path.to_path_buf(),
            source: e,
        }),
    }
}
