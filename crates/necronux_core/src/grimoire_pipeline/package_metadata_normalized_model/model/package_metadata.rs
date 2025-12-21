// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use std::collections::HashMap;

#[derive(Debug)]
pub struct NormalizedPackageMetadata {
    pub name: String,
    pub version: String,
    pub package_uri: String,
    pub package_zip_url: String,
    pub package_zip_checksums: NormalizedChecksums,
    pub dependencies: Option<HashMap<String, NormalizedDependency>>,
    pub description: Option<String>,
    pub authors: Option<Vec<String>>,
    pub website: Option<String>,
    pub documentation: Option<String>,
    pub source_code: Option<String>,
    pub source_code_url_scheme: Option<String>,
    pub license: String,
    pub license_text: Option<String>,
    pub issue_tracker: Option<String>,
}

#[derive(Debug)]
pub struct NormalizedChecksums {
    pub sha256: String,
}

#[derive(Debug)]
pub struct NormalizedDependency {
    pub uri: String,
    pub checksums: NormalizedChecksums,
}
