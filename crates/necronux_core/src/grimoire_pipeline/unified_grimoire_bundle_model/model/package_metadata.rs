// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use semver::Version;
use std::collections::HashMap;

#[derive(Debug)]
pub struct UnifiedPackageMetadata {
    pub name: String,
    pub version: Version,
    pub package_uri: String,
    pub package_zip_url: String,
    pub package_zip_checksums: UnifiedChecksums,
    pub dependencies: HashMap<String, UnifiedDependency>,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub website: Option<String>,
    pub documentation: Option<String>,
    pub source_code: Option<String>,
    pub source_code_url_scheme: Option<String>,
    pub license: String,
    pub license_text: Option<String>,
    pub issue_tracker: Option<String>,
}

#[derive(Debug)]
pub struct UnifiedChecksums {
    pub sha256: String,
}

#[derive(Debug)]
pub struct UnifiedDependency {
    pub uri: String,
    pub checksums: UnifiedChecksums,
}
