// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use std::path::PathBuf;
use thiserror::Error;

static NON_UTF8_FILE_NAME_CONTEXT_STR: &str = "Zip filename is not valid UTF-8";
static MISSING_ZIP_EXTENSION_CONTEXT_STR: &str = "Zip file does not have a '.zip' extension";
static INVALID_ZIP_FILE_NAME_FORMAT_CONTEXT_STR: &str =
    "Zip filename is not in 'package_name@package_version.zip' format";

#[derive(Debug, Error)]
#[error("Failed to introspect current grimoire")]
pub struct IntrospectCurrentGrimoireErrorWithContext {
    #[source]
    pub source: IntrospectCurrentGrimoireError,
}
#[derive(Debug, Error)]
pub enum IntrospectCurrentGrimoireError {
    #[error("{NON_UTF8_FILE_NAME_CONTEXT_STR}: '{path}'")]
    NonUtf8FileName { path: PathBuf },
    #[error("{INVALID_ZIP_FILE_NAME_FORMAT_CONTEXT_STR}: {file_name}")]
    InvalidZipFileNameFormat { file_name: String },
    #[error("{MISSING_ZIP_EXTENSION_CONTEXT_STR}: {file_name}")]
    MissingZipExtension { file_name: String },
    #[error("No .zip file found in current_grimoire directory: '{path}'")]
    NoZipFileFound { path: PathBuf },
    #[error("Multiple .zip files found in current_grimoire directory: '{path}'")]
    MultipleZipFiles { path: PathBuf },
    #[error(transparent)]
    PathError(#[from] necronux_utils::error::PathError),
    #[error(transparent)]
    FsError(#[from] necronux_utils::error::FsError),
}

#[derive(Debug, Error)]
#[error("Failed to introspect grimoire binding status")]
pub struct IntrospectGrimoireBindingStatusErrorWithContext {
    #[source]
    pub source: IntrospectGrimoireBindingStatusError,
}
#[derive(Debug, Error)]
pub enum IntrospectGrimoireBindingStatusError {
    #[error(transparent)]
    FsError(#[from] necronux_utils::error::FsError),
}

#[derive(Debug, Error)]
#[error(
    "Failed to check if storage backend supports the grimoire package source: '{package_zip_source}'"
)]
pub struct SupportsStorageBackendCheckErrorWithContext {
    pub package_zip_source: String,
    #[source]
    pub source: Box<SupportsStorageBackendCheckError>,
}
#[derive(Debug, Error)]
pub enum SupportsStorageBackendCheckError {
    #[error(transparent)]
    FsError(#[from] necronux_utils::error::FsError),
}

#[derive(Debug, Error)]
#[error("Failed to fetch the grimoire package source: '{package_zip_source}'")]
pub struct FetchGrimoirePackageSourceErrorWithContext {
    pub package_zip_source: String,
    #[source]
    pub source: Box<FetchGrimoirePackageSourceError>,
}
#[derive(Debug, Error)]
pub enum FetchGrimoirePackageSourceError {
    #[error("Invalid grimoire package source path: '{path}'. Expected a .zip file")]
    NotAZipFile { path: PathBuf },
    #[error("Failed to get file name from grimoire package source path: '{path}'")]
    FileNameExtractionFailed { path: PathBuf },
    #[error("{NON_UTF8_FILE_NAME_CONTEXT_STR}: '{path}'")]
    NonUtf8FileName { path: PathBuf },
    #[error("{MISSING_ZIP_EXTENSION_CONTEXT_STR}: {file_name}")]
    MissingZipExtension { file_name: String },
    #[error("{INVALID_ZIP_FILE_NAME_FORMAT_CONTEXT_STR}: {file_name}")]
    InvalidZipFileNameFormat { file_name: String },
    #[error(transparent)]
    PathError(#[from] necronux_utils::error::PathError),
    #[error(transparent)]
    FsError(#[from] necronux_utils::error::FsError),
}

#[derive(Debug, Error)]
#[error(
    "Failed to resolve storage backend for the grimoire package source: '{package_zip_source}'"
)]
pub struct ResolveStorageBackendErrorWithContext {
    pub package_zip_source: String,
    #[source]
    pub source: ResolveStorageBackendError,
}
#[derive(Debug, Error)]
pub enum ResolveStorageBackendError {
    #[error("Unsupported grimoire package source path or URL: '{package_zip_source}'")]
    UnsupportedSource { package_zip_source: String },
    #[error(transparent)]
    SupportsStorageBackendCheckError(#[from] SupportsStorageBackendCheckErrorWithContext),
}
