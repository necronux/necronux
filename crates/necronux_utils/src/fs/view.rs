// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::FsError;
use necronux_macros::trace_instrument;
use std::{
    fs::{File, ReadDir},
    path::Path,
    result as stdrt,
};
use tracing::debug;

#[trace_instrument(level = "debug", fields(label = %label, path = %path.display()))]
pub fn open_file(path: &Path, label: &str) -> stdrt::Result<File, FsError> {
    debug!(
        label = %label,
        path = %path.display(),
        "Attempting to open file..."
    );

    match std::fs::File::open(path) {
        Ok(file) => {
            debug!(
                label = %label,
                path = %path.display(),
                "Successfully opened file"
            );
            Ok(file)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(FsError::OpenFileError {
            label: label.to_string(),
            path: path.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Path does not exist when attempting to open file",
            ),
        }),
        Err(e) => Err(FsError::OpenFileError {
            path: path.to_path_buf(),
            label: label.to_string(),
            source: e,
        }),
    }
}

#[trace_instrument(level = "debug", fields(label = %label, path = %path.display()))]
pub fn read_to_string(path: &Path, label: &str) -> stdrt::Result<String, FsError> {
    debug!(
        label = %label,
        path = %path.display(),
        "Attempting to read contents of file...",
    );

    match std::fs::read_to_string(path) {
        Ok(contents) => {
            debug!(
                label = %label,
                path = %path.display(),
                "Successfully read contents of file",
            );
            Ok(contents)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(FsError::ReadFileError {
            label: label.to_string(),
            path: path.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Path does not exist when attempting to read contents of file",
            ),
        }),
        Err(e) => Err(FsError::ReadFileError {
            path: path.to_path_buf(),
            label: label.to_string(),
            source: e,
        }),
    }
}

#[trace_instrument(level = "debug", fields(label = %label, path = %path.display()))]
pub fn read_dir(path: &Path, label: &str) -> stdrt::Result<ReadDir, FsError> {
    debug!(
        label = %label,
        path = %path.display(),
        "Attempting to read contents of directory...",
    );

    match std::fs::read_dir(path) {
        Ok(iterator) => {
            debug!(
                label = %label,
                path = %path.display(),
                "Successfully read contents of directory",
            );
            Ok(iterator)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(FsError::ReadDirError {
            label: label.to_string(),
            path: path.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Path does not exist when attempting to read contents of directory",
            ),
        }),
        Err(e) => Err(FsError::ReadDirError {
            path: path.to_path_buf(),
            label: label.to_string(),
            source: e,
        }),
    }
}
