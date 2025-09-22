// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::LocalBackend;
use crate::error::{
    FetchGrimoirePackageSourceErrorWithContext, SupportsStorageBackendCheckErrorWithContext,
};
use std::{path::PathBuf, result as stdrt};

pub struct FetchedGrimoire {
    pub package_name: String,
    pub package_version: String,
    pub package_zip_source: String,
    pub fetched_package_zip_path: PathBuf,
    pub storage_backend: String,
}

pub trait StorageBackend {
    fn name(&self) -> &str;

    fn supports(
        &self,
        source: &str,
    ) -> stdrt::Result<bool, SupportsStorageBackendCheckErrorWithContext>;

    fn fetch(
        &self,
        source: &str,
    ) -> stdrt::Result<FetchedGrimoire, FetchGrimoirePackageSourceErrorWithContext>;
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

    fn supports(
        &self,
        source: &str,
    ) -> stdrt::Result<bool, SupportsStorageBackendCheckErrorWithContext> {
        match self {
            StorageBackendKind::Local(b) => b.supports(source),
        }
    }

    fn fetch(
        &self,
        source: &str,
    ) -> stdrt::Result<FetchedGrimoire, FetchGrimoirePackageSourceErrorWithContext> {
        match self {
            StorageBackendKind::Local(b) => b.fetch(source),
        }
    }
}
