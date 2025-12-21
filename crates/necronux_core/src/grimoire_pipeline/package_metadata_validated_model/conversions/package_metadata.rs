// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::ValidateGrimoireError,
    package_metadata_normalized_model::model::{
        NormalizedChecksums, NormalizedDependency, NormalizedPackageMetadata,
    },
    package_metadata_validated_model::model::{
        ValidatedChecksums, ValidatedDependency, ValidatedPackageMetadata,
    },
};
use std::{collections::HashMap, result as stdrt};

impl TryFrom<NormalizedPackageMetadata> for ValidatedPackageMetadata {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedPackageMetadata) -> stdrt::Result<Self, Self::Error> {
        let parent_object = "PklProject file";
        Ok(Self {
            name: s.name,
            version: semver::Version::parse(&s.version).map_err(|e| {
                ValidateGrimoireError::InvalidFieldValueSemverError {
                    field_name: "version".to_string(),
                    value: s.version.clone(),
                    parent_object_name: parent_object.to_string(),
                    source: e,
                }
            })?,
            package_uri: s.package_uri,
            package_zip_url: s.package_zip_url,
            package_zip_checksums: s.package_zip_checksums.try_into()?,
            dependencies: s
                .dependencies
                .map(|d| {
                    d.into_iter()
                        .map(|(k, v)| Ok((k, v.try_into?)))
                        .collect::<Result<HashMap<_, _>, _>>()
                })
                .transpose()?,
            description: s.description,
            authors: s.authors,
            website: s.website,
            documentation: s.documentation,
            source_code: s.source_code,
            source_code_url_scheme: s.source_code_url_scheme,
            license: {
                let license_str = s.license;

                if spdx::Expression::parse(&license_str).is_err() {
                    // Unknown license -> must have license_text
                    if s.license_text.is_none() {
                        return Err(ValidateGrimoireError::MissingRequiredField {
                                field_name: "licenseText".to_string(),
                                parent_object_name: parent_object.to_string(),
                                reason: "`licenseText` required when `license` is not a recognized SPDX identifier".to_string(),
                            });
                    }
                }
                license_str
            },
            license_text: s.license_text,
            issue_tracker: s.issue_tracker,
        })
    }
}

impl TryFrom<NormalizedChecksums> for ValidatedChecksums {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedChecksums) -> stdrt::Result<Self, Self::Error> {
        Ok(Self { sha256: s.sha256 })
    }
}

impl TryFrom<NormalizedDependency> for ValidatedDependency {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedDependency) -> stdrt::Result<Self, Self::Error> {
        Ok(Self {
            uri: s.uri,
            checksums: s.checksums.try_into()?,
        })
    }
}
