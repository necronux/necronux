// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{LocalBackend, StorageBackend, StorageBackendKind};
use crate::error::{PkgError, ResolveStorageBackendError as ResStorBackError};
use necronux_utils::trace_instrument;
use std::result as stdrt;
use tracing::debug;

#[trace_instrument(level = "debug", fields(source = %source))]
pub fn resolve_storage_backend(source: &str) -> stdrt::Result<StorageBackendKind, PkgError> {
    debug!(
        source = %source,
        "Resolving storage backend for the grimoire package source..."
    );

    fn resolve_storage_backend_inner(
        source: &str,
    ) -> stdrt::Result<StorageBackendKind, ResStorBackError> {
        let backends = [StorageBackendKind::Local(LocalBackend {})];

        for backend in backends {
            if backend.supports(source).map_err(|e| {
                ResStorBackError::SupportsStorageBackendCheckError {
                    package_zip_source: source.to_string(),
                    source: Box::new(e),
                }
            })? {
                return Ok(backend);
            }
        }

        Err(ResStorBackError::UnsupportedGrimoirePackageSourceError {
            package_zip_source: source.to_string(),
        })
    }

    let backend = resolve_storage_backend_inner(source).map_err(|e| {
        PkgError::ResolveStorageBackendError {
            package_zip_source: source.to_string(),
            source: Box::new(e),
        }
    })?;

    debug!(
        backend = %backend.name(),
        source = %source,
        "Successfully resolved storage backend '{}' for the grimoire package source",
        backend.name()
    );
    Ok(backend)
}
