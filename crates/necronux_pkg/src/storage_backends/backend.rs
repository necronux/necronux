// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::LocalBackend;
use crate::error::Result;
use std::path::PathBuf;

pub struct FetchedGrimoire {
    pub package_name: String,
    pub package_version: String,
    pub package_zip_source: String,
    pub fetched_package_zip_path: PathBuf,
    pub storage_backend: String,
}

pub trait StorageBackend {
    fn name(&self) -> &str;

    fn supports(&self, source: &str) -> Result<bool>;

    fn fetch(&self, source: &str) -> Result<FetchedGrimoire>;
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

    fn supports(&self, source: &str) -> Result<bool> {
        match self {
            StorageBackendKind::Local(b) => b.supports(source),
        }
    }

    fn fetch(&self, source: &str) -> Result<FetchedGrimoire> {
        match self {
            StorageBackendKind::Local(b) => b.fetch(source),
        }
    }
}
