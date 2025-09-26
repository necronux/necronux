// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::LocalBackend;
use crate::error::{
    FetchGrimoirePackageSourceError, FetchGrimoirePackageSourceErrorWithContext,
    SupportsStorageBackendCheckError, SupportsStorageBackendCheckErrorWithContext,
};
use necronux_utils::trace_instrument;
use std::{path::PathBuf, result as stdrt};
use tracing::debug;

pub struct FetchedGrimoire {
    pub package_name: String,
    pub package_version: String,
    pub package_zip_source: String,
    pub fetched_package_zip_path: PathBuf,
    pub storage_backend: String,
}

pub trait StorageBackend {
    fn name(&self) -> &str;

    #[trace_instrument(level = "debug", name = "supports_storage_backend_check", skip(self), fields(source = %source))]
    fn supports(
        &self,
        source: &str,
    ) -> stdrt::Result<bool, SupportsStorageBackendCheckErrorWithContext> {
        let backend_name = self.name();

        debug!(
            backend = %backend_name,
            source = %source,
            "Checking if {backend_name} storage backend supports the grimoire package source...",
        );

        let (supports, support_str) = self.supports_inner(source).map_err(|e| {
            SupportsStorageBackendCheckErrorWithContext {
                package_zip_source: source.to_string(),
                source: Box::new(e),
            }
        })?;

        debug!(
            backend = %backend_name,
            supports = %support_str,
            "{} storage backend support for the grimoire package source: {}",
            necronux_utils::string::capitalize_first(backend_name),
            support_str
        );
        Ok(supports)
    }

    fn supports_inner(
        &self,
        source: &str,
    ) -> stdrt::Result<(bool, String), SupportsStorageBackendCheckError>;

    #[trace_instrument(level = "debug", name = "fetch_storage_backend", skip(self), fields(source = %source))]
    fn fetch(
        &self,
        source: &str,
    ) -> stdrt::Result<FetchedGrimoire, FetchGrimoirePackageSourceErrorWithContext> {
        let backend_name = self.name();

        debug!(
            backend = backend_name,
            source = %source,
            "Fetching grimoire package source with {backend_name} backend...",
        );

        let fetched =
            self.fetch_inner(source)
                .map_err(|e| FetchGrimoirePackageSourceErrorWithContext {
                    package_zip_source: source.to_string(),
                    source: e,
                })?;

        debug!(
            backend = %backend_name,
            source = %source,
            fetched_package_zip_path = %fetched.fetched_package_zip_path.display(),
            "Successfully fetched grimoire package source with {backend_name} backend",
        );
        Ok(fetched)
    }

    fn fetch_inner(
        &self,
        source: &str,
    ) -> stdrt::Result<FetchedGrimoire, FetchGrimoirePackageSourceError>;
}

pub enum StorageBackendKind {
    Local(LocalBackend),
}

impl StorageBackend for StorageBackendKind {
    fn name(&self) -> &str {
        match self {
            StorageBackendKind::Local(b) => b.name(),
        }
    }

    fn supports_inner(
        &self,
        source: &str,
    ) -> stdrt::Result<(bool, String), SupportsStorageBackendCheckError> {
        match self {
            StorageBackendKind::Local(b) => b.supports_inner(source),
        }
    }

    fn fetch_inner(
        &self,
        source: &str,
    ) -> stdrt::Result<FetchedGrimoire, FetchGrimoirePackageSourceError> {
        match self {
            StorageBackendKind::Local(b) => b.fetch_inner(source),
        }
    }
}
