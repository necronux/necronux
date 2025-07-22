// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{LocalBackend, StorageBackend, StorageBackendKind};
use crate::error::{PkgError, Result};
use tracing::{debug, info};

pub fn resolve_storage_backend(source: &str) -> Result<StorageBackendKind> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("resolve_storage_backend", source = source).entered();

    info!("Resolving storage backend...");

    fn resolve_storage_backend_inner(source: &str) -> Result<StorageBackendKind> {
        let backends = [StorageBackendKind::Local(LocalBackend {})];

        for backend in backends {
            if backend.supports(source)? {
                debug!(
                    "Found supported storage backend '{}' for the grimoire package source: {source}",
                    backend.name(),
                );
                return Ok(backend);
            }
        }

        Err(PkgError::UnsupportedGrimoirePackageSource {
            package_zip_source: source.to_string(),
        })
    }

    resolve_storage_backend_inner(source).map_err(|e| PkgError::ResolveStorageBackendError {
        package_zip_source: source.to_string(),
        source: Box::new(e),
    })
}
