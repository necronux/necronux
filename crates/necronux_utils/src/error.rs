// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Failed to extract {zip_path_label} zip file from '{zip_path}' to {dest_path}")]
pub struct ExtractZipErrorWithContext {
    pub zip_path_label: String,
    pub zip_path: PathBuf,
    pub dest_path: PathBuf,
    #[source]
    pub source: Box<ExtractZipError>,
}
#[derive(Debug, Error)]
pub enum ExtractZipError {
    #[error("Failed to read and extract {label} file at '{path}'")]
    ZipError {
        label: String,
        path: PathBuf,
        #[source]
        source: zip::result::ZipError,
    },
    #[error(transparent)]
    FsError(#[from] FsError),
    #[error(transparent)]
    IoError(#[from] IoError),
}

#[derive(Debug, Error)]
pub enum PathError {
    #[error("Failed to determine platform-specific project directory for Necronux")]
    ProjectDirResolutionFailed,
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
    #[error("Failed to create {label} file at '{path}'")]
    CreateFileError {
        label: String,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Failed to create {label} directory at '{path}'")]
    CreateDirError {
        label: String,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to open {label} file at '{path}'")]
    OpenFileError {
        label: String,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Failed to open {label} directory at '{path}'")]
    OpenDirError {
        label: String,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to read {label} file at '{path}'")]
    ReadFileError {
        label: String,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Failed to read {label} directory at '{path}'")]
    ReadDirError {
        label: String,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to copy from {from_path_label} file from '{from_path}' to '{to_path}'")]
    CopyFileError {
        from_path_label: String,
        from_path: PathBuf,
        to_path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to write {label} file at '{path}'")]
    WriteFileError {
        label: String,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to rename {old_path_label} directory from '{old_path}' to '{new_path}'")]
    RenameDirError {
        old_path_label: String,
        old_path: PathBuf,
        new_path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to delete {label} directory at '{path}'")]
    RemoveDirError {
        label: String,
        path: PathBuf,
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
