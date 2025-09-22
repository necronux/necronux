// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{LocalBackend, StorageBackend, StorageBackendKind};
use crate::error::{ResolveStorageBackendError, ResolveStorageBackendErrorWithContext};
use necronux_utils::trace_instrument;
use std::result as stdrt;
use tracing::debug;

#[trace_instrument(level = "debug", fields(source = %source))]
pub fn resolve_storage_backend(
    source: &str,
) -> stdrt::Result<StorageBackendKind, ResolveStorageBackendErrorWithContext> {
    debug!(
        source = %source,
        "Resolving storage backend for the grimoire package source..."
    );

    let backend = resolve_storage_backend_inner(source).map_err(|e| {
        ResolveStorageBackendErrorWithContext {
            package_zip_source: source.to_string(),
            source: e,
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

fn resolve_storage_backend_inner(
    source: &str,
) -> stdrt::Result<StorageBackendKind, ResolveStorageBackendError> {
    let backends = [StorageBackendKind::Local(LocalBackend {})];

    for backend in backends {
        if backend.supports(source)? {
            return Ok(backend);
        }
    }

    Err(ResolveStorageBackendError::UnsupportedSource {
        package_zip_source: source.to_string(),
    })
}
