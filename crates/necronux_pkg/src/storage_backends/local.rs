// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::FetchedGrimoire;
use crate::{
    StorageBackend,
    error::{FetchGrimoirePackageSourceError, SupportsStorageBackendCheckError},
};
use std::{path::Path, result as stdrt};
use tracing::debug;

#[derive(Debug)]
pub struct LocalBackend;

impl StorageBackend for LocalBackend {
    fn name(&self) -> &str {
        "local"
    }

    fn supports_inner(
        &self,
        source: &str,
    ) -> stdrt::Result<(bool, String), SupportsStorageBackendCheckError> {
        let path = Path::new(source);
        let support = necronux_utils::fs::file_exists(path, "grimoire package source")?;
        let support_str = if support { "supported" } else { "unsupported" };
        let backend_name = self.name();
        debug!(
            backend = %backend_name,
            source = %source,
            support = %support,
            "{} storage backend support for the grimoire package source: {}",
            necronux_utils::string::capitalize_first(backend_name),
            support
        );
        Ok((support, support_str.to_string()))
    }

    fn fetch_inner(
        &self,
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
            storage_backend: self.name().to_string(),
        };
        Ok(fetched)
    }
}
