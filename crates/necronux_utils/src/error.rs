// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use std::path::PathBuf;
use thiserror::Error;
use zip::result::ZipError as ZipRsError;

// Crate level errors

#[derive(Debug, Error)]
pub enum ZipError {
    #[error("Failed to extract {zip_path_label} zip file from '{zip_path}' to {dest_path}")]
    ExtractZipError {
        zip_path_label: String,
        zip_path: PathBuf,
        dest_path: PathBuf,
        #[source]
        source: Box<ExtractZipError>,
    },
}

#[derive(Debug, Error)]
pub enum PathError {
    #[error("Failed to determine platform-specific project directory for Necronux")]
    NecronuxProjectDirError,
}

#[derive(Debug, Error)]
pub enum IoError {
    #[error("Failed to copy from {reader_label} to {writer_label}")]
    CopyStreamError {
        reader_label: String,
        writer_label: String,
        #[source]
        source: std::io::Error,
    },
}

#[derive(Debug, Error)]
pub enum FsError {
    #[error("Failed to create {label} {target} at '{path}'")]
    CreateError {
        label: String,
        path: PathBuf,
        target: FsTarget,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to open {label} {target} at '{path}'")]
    OpenError {
        label: String,
        path: PathBuf,
        target: FsTarget,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to read {label} {target} at '{path}'")]
    ReadError {
        label: String,
        path: PathBuf,
        target: FsTarget,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to copy from {from_path_label} {target} from '{from_path}' to '{to_path}'")]
    CopyError {
        from_path_label: String,
        from_path: PathBuf,
        to_path: PathBuf,
        target: FsTarget,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write {label} {target} at '{path}'")]
    WriteError {
        label: String,
        path: PathBuf,
        target: FsTarget,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to rename {old_path_label} {target} from '{old_path}' to '{new_path}'")]
    RenameError {
        old_path_label: String,
        old_path: PathBuf,
        new_path: PathBuf,
        target: FsTarget,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to delete {label} {target} at '{path}'")]
    RemoveError {
        label: String,
        path: PathBuf,
        target: FsTarget,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to check path existence of {label} at '{path}'")]
    CheckPathExistsError {
        label: String,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to set permissions on {label} at '{path}'")]
    SetPermissionsError {
        label: String,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

// Function level errors

#[derive(Debug, Error)]
pub enum ExtractZipError {
    #[error("Failed to read {label} file at '{path}'")]
    ReadError {
        label: String,
        path: PathBuf,
        #[source]
        source: ZipRsError,
    },

    #[error(transparent)]
    IoError(#[from] IoError),

    #[error(transparent)]
    FsError(#[from] FsError),

    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
}

// Helpers

#[derive(Debug)]
pub enum FsTarget {
    Directory,
    File,
}

impl std::fmt::Display for FsTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FsTarget::Directory => write!(f, "directory"),
            FsTarget::File => write!(f, "file"),
        }
    }
}
