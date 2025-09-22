// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::FetchedGrimoire;
use crate::{
    StorageBackend,
    error::{
        FetchGrimoirePackageSourceError, FetchGrimoirePackageSourceErrorWithContext,
        SupportsStorageBackendCheckError, SupportsStorageBackendCheckErrorWithContext,
    },
};
use necronux_utils::trace_instrument;
use std::{path::Path, result as stdrt};
use tracing::debug;

#[derive(Debug)]
pub struct LocalBackend;

impl StorageBackend for LocalBackend {
    fn name(&self) -> &str {
        "local"
    }

    #[trace_instrument(level = "debug", name = "LocalBackend::supports", skip(self), fields(source = %source))]
    fn supports(
        &self,
        source: &str,
    ) -> stdrt::Result<bool, SupportsStorageBackendCheckErrorWithContext> {
        debug!(
            backend = %self.name(),
            source = %source,
            "Checking if {} storage backend supports the grimoire package source...",
            self.name()
        );

        let (supports, support_str) = supports_inner(self, source).map_err(|e| {
            SupportsStorageBackendCheckErrorWithContext {
                package_zip_source: source.to_string(),
                source: e,
            }
        })?;

        debug!(
            backend = %self.name(),
            supports = %support_str,
            "{} storage backend support for the grimoire package source: {}",
            necronux_utils::string::capitalize_first(self.name()),
            support_str
        );
        Ok(supports)
    }

    #[trace_instrument(level = "debug", name = "LocalBackend::fetch", skip(self), fields(source = %source))]
    fn fetch(
        &self,
        source: &str,
    ) -> stdrt::Result<FetchedGrimoire, FetchGrimoirePackageSourceErrorWithContext> {
        debug!(
            backend = %self.name(),
            source = %source,
            "Fetching grimoire package source with {} backend...",
            self.name()
        );

        let fetched =
            fetch_inner(self, source).map_err(|e| FetchGrimoirePackageSourceErrorWithContext {
                package_zip_source: source.to_string(),
                source: e,
            })?;

        debug!(
            backend = %self.name(),
            source = %source,
            fetched_package_zip_path = %fetched.fetched_package_zip_path.display(),
            "Successfully fetched grimoire package source with {} backend",
            self.name()
        );
        Ok(fetched)
    }
}

fn supports_inner(
    backend: &LocalBackend,
    source: &str,
) -> stdrt::Result<(bool, String), SupportsStorageBackendCheckError> {
    let path = Path::new(source);
    let support = necronux_utils::fs::file_exists(path, "grimoire package source")?;
    let support_str = if support { "supported" } else { "unsupported" };
    debug!(
        backend = %backend.name(),
        source = %source,
        support = %support,
        "{} storage backend support for the grimoire package source: {}",
        necronux_utils::string::capitalize_first(backend.name()),
        support
    );
    Ok((support, support_str.to_string()))
}

fn fetch_inner(
    backend: &LocalBackend,
    source: &str,
) -> stdrt::Result<FetchedGrimoire, FetchGrimoirePackageSourceError> {
    let source_path = Path::new(source);
    if !necronux_utils::fs::file_exists(source_path, "grimoire package source")? {
        return Err(FetchGrimoirePackageSourceError::NotAZipFile {
            path: source_path.to_path_buf(),
        });
    }

    let zip_file_name = source_path.file_name().ok_or_else(|| {
        FetchGrimoirePackageSourceError::FileNameExtractionFailed {
            path: source_path.to_path_buf(),
        }
    })?;
    let dest_dir = necronux_utils::paths::current_grimoire_path()?;
    let dest_zip_path = dest_dir.join(zip_file_name);

    necronux_utils::fs::create_dir_all(&dest_dir, "current grimoire")?;
    necronux_utils::fs::copy(source_path, "grimoire package source", &dest_zip_path)?;

    let zip_file_str =
        zip_file_name
            .to_str()
            .ok_or(FetchGrimoirePackageSourceError::NonUtf8FileName {
                path: source_path.to_path_buf(),
            })?;
    if !zip_file_str.ends_with(".zip") {
        return Err(FetchGrimoirePackageSourceError::MissingZipExtension {
            file_name: zip_file_str.to_string(),
        });
    }

    let base_name = &zip_file_str[..zip_file_str.len() - 4];
    let (package_name, package_version) = base_name.rsplit_once('@').ok_or_else(|| {
        FetchGrimoirePackageSourceError::InvalidZipFileNameFormat {
            file_name: zip_file_str.to_string(),
        }
    })?;

    let fetched = super::FetchedGrimoire {
        package_name: package_name.to_string(),
        package_version: package_version.to_string(),
        package_zip_source: source.to_string(),
        fetched_package_zip_path: dest_zip_path.to_path_buf(),
        storage_backend: backend.name().to_string(),
    };
    Ok(fetched)
}
