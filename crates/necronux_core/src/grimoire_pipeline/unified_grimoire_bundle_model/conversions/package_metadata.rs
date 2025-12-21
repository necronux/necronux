// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    package_metadata_validated_model::model::{
        ValidatedChecksums, ValidatedDependency, ValidatedPackageMetadata,
    },
    unified_grimoire_bundle_model::model::{
        UnifiedChecksums, UnifiedDependency, UnifiedPackageMetadata,
    },
};

impl From<ValidatedPackageMetadata> for UnifiedPackageMetadata {
    fn from(s: ValidatedPackageMetadata) -> Self {
        Self {
            name: s.name,
            version: s.version,
            package_uri: s.package_uri,
            package_zip_url: s.package_zip_url,
            package_zip_checksums: s.package_zip_checksums.into(),
            dependencies: s
                .dependencies
                .map(|d| d.into_iter().map(|(k, v)| (k, v.into())).collect())
                .unwrap_or_default(),
            description: s.description,
            authors: s.authors.unwrap_or_default(),
            website: s.website,
            documentation: s.documentation,
            source_code: s.source_code,
            source_code_url_scheme: s.source_code_url_scheme,
            license: s.license,
            license_text: s.license_text,
            issue_tracker: s.issue_tracker,
        }
    }
}

impl From<ValidatedChecksums> for UnifiedChecksums {
    fn from(s: ValidatedChecksums) -> Self {
        Self { sha256: s.sha256 }
    }
}

impl From<ValidatedDependency> for UnifiedDependency {
    fn from(s: ValidatedDependency) -> Self {
        Self {
            uri: s.uri,
            checksums: s.checksums.into(),
        }
    }
}
