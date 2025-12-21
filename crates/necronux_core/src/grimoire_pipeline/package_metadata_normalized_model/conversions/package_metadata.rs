// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::NormalizeGrimoireError,
    grimoire_pipeline::normalizers,
    package_metadata_normalized_model::model::{
        NormalizedChecksums, NormalizedDependency, NormalizedPackageMetadata,
    },
    package_metadata_parsed_model::model::{
        ParsedChecksums, ParsedDependency, ParsedPackageMetadata,
    },
};
use std::{collections::HashMap, result as stdrt};

impl TryFrom<ParsedPackageMetadata> for NormalizedPackageMetadata {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedPackageMetadata) -> stdrt::Result<Self, Self::Error> {
        let parent_object = "PklProject file";
        Ok(Self {
            name: normalizers::ensure_req_field_is_not_missing(s.name, "name", parent_object)?,
            version: normalizers::ensure_req_field_is_not_missing(
                s.version,
                "version",
                parent_object,
            )?,
            package_uri: normalizers::ensure_req_field_is_not_missing(
                s.package_uri,
                "packageUri",
                parent_object,
            )?,
            package_zip_url: normalizers::ensure_req_field_is_not_missing(
                s.package_zip_url,
                "packageZipUrl",
                parent_object,
            )?,
            package_zip_checksums: normalizers::ensure_req_field_is_not_missing(
                s.package_zip_checksums,
                "packageZipChecksums",
                parent_object,
            )?
            .try_into()?,
            dependencies: s
                .dependencies
                .map(|d| {
                    d.into_iter()
                        .map(|(k, v)| v.try_into().map(|nv| (k, nv)))
                        .collect::<Result<HashMap<_, _>, _>>()
                })
                .transpose()?,
            description: s.description,
            authors: s.authors,
            website: s.website,
            documentation: s.documentation,
            source_code: s.source_code,
            source_code_url_scheme: s.source_code_url_scheme,
            license: normalizers::ensure_req_field_is_not_missing(
                s.license,
                "license",
                parent_object,
            )?,
            license_text: s.license_text,
            issue_tracker: s.issue_tracker,
        })
    }
}

impl TryFrom<ParsedChecksums> for NormalizedChecksums {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedChecksums) -> stdrt::Result<Self, Self::Error> {
        let parent_object = "PklProject Checksums";
        Ok(Self {
            sha256: normalizers::ensure_req_field_is_not_missing(
                s.sha256,
                "sha256",
                parent_object,
            )?,
        })
    }
}

impl TryFrom<ParsedDependency> for NormalizedDependency {
    type Error = NormalizeGrimoireError;

    fn try_from(s: ParsedDependency) -> stdrt::Result<Self, Self::Error> {
        let parent_object = "PklProject Dependency";
        Ok(Self {
            uri: normalizers::ensure_req_field_is_not_missing(s.uri, "uri", parent_object)?,
            checksums: normalizers::ensure_req_field_is_not_missing(
                s.checksums,
                "checksums",
                parent_object,
            )?
            .try_into()?,
        })
    }
}
