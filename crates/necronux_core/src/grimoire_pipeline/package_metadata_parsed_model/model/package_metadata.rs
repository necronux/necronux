// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedPackageMetadata {
    // Option as RFB (See lib.rs)
    pub name: Option<String>,

    // Option as RFB (See lib.rs)
    pub version: Option<String>,

    // Option as RFB (See lib.rs)
    pub package_uri: Option<String>,

    // Option as RFB (See lib.rs)
    pub package_zip_url: Option<String>,

    // Option as RFB (See lib.rs)
    pub package_zip_checksums: Option<ParsedChecksums>,

    pub dependencies: Option<HashMap<String, ParsedDependency>>,

    pub description: Option<String>,

    pub authors: Option<Vec<String>>,

    pub website: Option<String>,

    pub documentation: Option<String>,

    pub source_code: Option<String>,

    pub source_code_url_scheme: Option<String>,

    // Option as RFB (See lib.rs)
    pub license: Option<String>,

    pub license_text: Option<String>,

    pub issue_tracker: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedChecksums {
    // Option as RFB (See lib.rs)
    pub sha256: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedDependency {
    // Option as RFB (See lib.rs)
    pub uri: Option<String>,

    // Option as RFB (See lib.rs)
    pub checksums: Option<ParsedChecksums>,
}
