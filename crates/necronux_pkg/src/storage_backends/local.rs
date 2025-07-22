// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::FetchedGrimoire;
use crate::{
    StorageBackend,
    error::{PkgError, Result},
};
use std::path::Path;
use tracing::{debug, info};

pub struct LocalBackend;

impl StorageBackend for LocalBackend {
    fn name(&self) -> &str {
        "local"
    }

    fn supports(&self, source: &str) -> Result<bool> {
        #[cfg(feature = "trace")]
        let _span =
            tracing::debug_span!("supports_local_storage_backend", source = source).entered();

        info!(
            "Checking if {} storage backend supports the grimoire package source...",
            self.name()
        );

        fn supports_inner(backend: &LocalBackend, source: &str) -> Result<bool> {
            let path = Path::new(source);
            let result = necronux_utils::fs::file_exists(path, "grimoire package source")?;
            debug!(
                "{} storage backend support for the grimoire package source: {}",
                necronux_utils::string::capitalize_first(backend.name()),
                result
            );
            Ok(result)
        }

        supports_inner(self, source).map_err(|e| PkgError::SupportsStorageBackendCheckError {
            package_zip_source: source.to_string(),
            source: Box::new(e),
        })
    }

    fn fetch(&self, source: &str) -> Result<FetchedGrimoire> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("fetch_grimoire_pkg_local", source = source).entered();

        info!(
            "Fetching grimoire package source with {} backend...",
            self.name()
        );

        fn fetch_from_local_if_exists(
            backend: &LocalBackend,
            source: &str,
        ) -> Result<FetchedGrimoire> {
            let source_path = Path::new(source);
            if !necronux_utils::fs::file_exists(source_path, "grimoire package source")? {
                return Err(PkgError::NonZipGrimoirePackageSourcePath {
                    path: source_path.to_path_buf(),
                });
            }

            let zip_file_name = source_path.file_name().ok_or_else(|| {
                PkgError::GetFileNameGrimoirePackageSourceError {
                    path: source_path.to_path_buf(),
                }
            })?;

            let dest_dir = necronux_utils::paths::current_grimoire_path()?;
            let dest_zip_path = dest_dir.join(zip_file_name);

            necronux_utils::fs::create_dir_all(&dest_dir, "current grimoire")?;
            necronux_utils::fs::copy_if_exists(
                source_path,
                "grimoire package source",
                &dest_zip_path,
            )?;

            let zip_file_str = zip_file_name.to_str().ok_or(PkgError::NonUtf8FileName {
                path: source_path.to_path_buf(),
            })?;

            if !zip_file_str.ends_with(".zip") {
                return Err(PkgError::MissingZipExtension {
                    file_name: zip_file_str.to_string(),
                });
            }

            let base_name = &zip_file_str[..zip_file_str.len() - 4];

            let (package_name, package_version) =
                base_name
                    .rsplit_once('@')
                    .ok_or_else(|| PkgError::InvalidZipFileNameFormat {
                        file_name: zip_file_str.to_string(),
                    })?;

            let fetched = super::FetchedGrimoire {
                package_name: package_name.to_string(),
                package_version: package_version.to_string(),
                package_zip_source: source.to_string(),
                fetched_package_zip_path: dest_zip_path.to_path_buf(),
                storage_backend: backend.name().to_string(),
            };

            debug!(
                "Successfully fetched grimoire package source ({source}) at '{}' with backend: {}",
                dest_zip_path.display(),
                backend.name()
            );
            Ok(fetched)
        }

        fetch_from_local_if_exists(self, source).map_err(|e| {
            PkgError::FetchGrimoirePackageSourceError {
                package_zip_source: source.to_string(),
                source: Box::new(e),
            }
        })
    }
}
