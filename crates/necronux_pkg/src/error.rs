// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, PkgError>;

#[derive(Debug, Error)]
pub enum PkgError {
    #[error("Failed to fetch grimoire package source: '{package_zip_source}'")]
    FetchGrimoirePackageSourceError {
        package_zip_source: String,
        #[source]
        source: Box<PkgError>,
    },

    #[error(
        "Failed to resolve storage backend for grimoire package source: '{package_zip_source}'"
    )]
    ResolveStorageBackendError {
        package_zip_source: String,
        #[source]
        source: Box<PkgError>,
    },

    #[error(
        "Failed to check if storage backend supports the grimoire package source: '{package_zip_source}'"
    )]
    SupportsStorageBackendCheckError {
        package_zip_source: String,
        #[source]
        source: Box<PkgError>,
    },

    #[error("Failed to introspect current grimoire")]
    IntrospectCurrentGrimoireError {
        #[source]
        source: Box<PkgError>,
    },

    #[error("Failed to introspect grimoire binding status")]
    IntrospectGrimoireBindingStatusError {
        #[source]
        source: Box<PkgError>,
    },

    #[error("Unsupported grimoire package source path or URL: '{package_zip_source}'")]
    UnsupportedGrimoirePackageSource { package_zip_source: String },

    #[error("Invalid grimoire package source path: '{path}'. Expected a .zip file")]
    NonZipGrimoirePackageSourcePath { path: PathBuf },

    #[error("Failed to get file name from grimoire package source path: '{path}'")]
    GetFileNameGrimoirePackageSourceError { path: PathBuf },

    #[error("Zip filename is not valid UTF-8: '{path}'")]
    NonUtf8FileName { path: PathBuf },

    #[error("Zip filename is not in '<name>@<version>.zip' format: {file_name}")]
    InvalidZipFileNameFormat { file_name: String },

    #[error("Zip file does not have a '.zip' extension: {file_name}")]
    MissingZipExtension { file_name: String },

    #[error("No .zip file found in current_grimoire directory")]
    NoZipFound,

    #[error("Multiple .zip files found in current_grimoire directory")]
    MultipleZips,

    #[error(transparent)]
    Zip(#[from] necronux_utils::error::ZipError),

    #[error(transparent)]
    Path(#[from] necronux_utils::error::PathError),

    #[error(transparent)]
    Fs(#[from] necronux_utils::error::FsError),
}
