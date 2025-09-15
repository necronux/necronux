// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use std::path::PathBuf;
use thiserror::Error;

// Crate level error

#[derive(Debug, Error)]
pub enum PkgError {
    #[error("Failed to introspect current grimoire")]
    IntrospectCurrentGrimoireError {
        #[source]
        source: IntrospectCurrentGrimoireError,
    },

    #[error("Failed to introspect grimoire binding status")]
    IntrospectGrimoireBindingStatusError {
        #[source]
        source: IntrospectGrimoireBindingStatusError,
    },

    #[error("Failed to fetch the grimoire package source: '{package_zip_source}'")]
    FetchGrimoirePackageSourceError {
        package_zip_source: String,
        #[source]
        source: Box<FetchGrimoirePackageSourceError>,
    },

    #[error(
        "Failed to resolve storage backend for the grimoire package source: '{package_zip_source}'"
    )]
    ResolveStorageBackendError {
        package_zip_source: String,
        #[source]
        source: Box<ResolveStorageBackendError>,
    },
}

// Function level errors

#[derive(Debug, Error)]
pub enum IntrospectCurrentGrimoireError {
    #[error("{NON_UTF8_FILE_NAME_ERROR_STR}: '{path}'")]
    NonUtf8FileNameError { path: PathBuf },

    #[error("{INVALID_ZIP_FILE_NAME_FORMAT_ERROR_STR}: {file_name}")]
    InvalidZipFileNameFormatError { file_name: String },

    #[error("{MISSING_ZIP_EXTENSION_ERROR_STR}: {file_name}")]
    MissingZipExtensionError { file_name: String },

    #[error("No .zip file found in current_grimoire directory: '{path}'")]
    NoZipFoundError { path: PathBuf },

    #[error("Multiple .zip files found in current_grimoire directory: '{path}'")]
    MultipleZipsError { path: PathBuf },

    #[error(transparent)]
    PathError(#[from] necronux_utils::error::PathError),

    #[error(transparent)]
    FsError(#[from] necronux_utils::error::FsError),
}

#[derive(Debug, Error)]
pub enum IntrospectGrimoireBindingStatusError {
    #[error(transparent)]
    FsError(#[from] necronux_utils::error::FsError),
}

#[derive(Debug, Error)]
pub enum SupportsStorageBackendCheckError {
    #[error(transparent)]
    FsError(#[from] necronux_utils::error::FsError),
}

#[derive(Debug, Error)]
pub enum FetchGrimoirePackageSourceError {
    #[error("Invalid grimoire package source path: '{path}'. Expected a .zip file")]
    NonZipGrimoirePackageSourcePathError { path: PathBuf },

    #[error("Failed to get file name from grimoire package source path: '{path}'")]
    GetFileNameGrimoirePackageSourceError { path: PathBuf },

    #[error("{NON_UTF8_FILE_NAME_ERROR_STR}: '{path}'")]
    NonUtf8FileNameError { path: PathBuf },

    #[error("{MISSING_ZIP_EXTENSION_ERROR_STR}: {file_name}")]
    MissingZipExtensionError { file_name: String },

    #[error("{INVALID_ZIP_FILE_NAME_FORMAT_ERROR_STR}: {file_name}")]
    InvalidZipFileNameFormatError { file_name: String },

    #[error(transparent)]
    PathError(#[from] necronux_utils::error::PathError),

    #[error(transparent)]
    FsError(#[from] necronux_utils::error::FsError),
}

#[derive(Debug, Error)]
pub enum ResolveStorageBackendError {
    #[error("Unsupported grimoire package source path or URL: '{package_zip_source}'")]
    UnsupportedGrimoirePackageSourceError { package_zip_source: String },

    #[error(
        "Failed to check if storage backend supports the grimoire package source: '{package_zip_source}'"
    )]
    SupportsStorageBackendCheckError {
        package_zip_source: String,
        #[source]
        source: Box<SupportsStorageBackendCheckError>,
    },
}

// Helpers

static NON_UTF8_FILE_NAME_ERROR_STR: &str = "Zip filename is not valid UTF-8";
static MISSING_ZIP_EXTENSION_ERROR_STR: &str = "Zip file does not have a '.zip' extension";
static INVALID_ZIP_FILE_NAME_FORMAT_ERROR_STR: &str =
    "Zip filename is not in 'package_name@package_version.zip' format";
