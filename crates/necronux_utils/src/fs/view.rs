// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::Result;
use crate::error::FsError;
use std::{fs::File, path::Path};
use tracing::debug;

pub fn open_file_if_exists(path: &Path, label: &str) -> Result<File> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("open_file", label = label).entered();

    if !super::path_exists(path, label)? {
        return Err(FsError::OpenFileError {
            label: label.to_string(),
            path: path.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Path does not exist when attempting to open file",
            ),
        });
    }

    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("open_file_op", label = label).entered();
    let file = std::fs::File::open(path).map_err(|e| FsError::OpenFileError {
        path: path.to_path_buf(),
        label: label.to_string(),
        source: e,
    })?;
    debug!("Successfully opened {label} file at '{}'", path.display());
    Ok(file)
}

pub fn read_to_string_if_exists(path: &Path, label: &str) -> Result<String> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("read_to_string", label = label).entered();

    if !super::path_exists(path, label)? {
        return Err(FsError::ReadToStringError {
            label: label.to_string(),
            path: path.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Path does not exist when attempting to read to string",
            ),
        });
    }

    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("read_to_string_op", label = label).entered();
    let contents = std::fs::read_to_string(path).map_err(|e| FsError::ReadToStringError {
        path: path.to_path_buf(),
        label: label.to_string(),
        source: e,
    })?;
    debug!(
        "Successfully read to string {label} file at '{}'",
        path.display()
    );
    Ok(contents)
}

pub fn read_dir_if_exists(path: &Path, label: &str) -> Result<std::fs::ReadDir> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("read_dir", label = label).entered();

    if !super::path_exists(path, label)? {
        return Err(FsError::ReadDirError {
            label: label.to_string(),
            path: path.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Path does not exist when attempting to read directory",
            ),
        });
    }

    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("read_dir_op", label = label).entered();
    let iterator = std::fs::read_dir(path).map_err(|e| FsError::ReadDirError {
        path: path.to_path_buf(),
        label: label.to_string(),
        source: e,
    })?;
    debug!(
        "Successfully read {label} directory at '{}'",
        path.display()
    );
    Ok(iterator)
}
