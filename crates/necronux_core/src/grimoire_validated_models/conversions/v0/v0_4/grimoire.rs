// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::ValidateGrimoireError,
    grimoire_normalized_models::models::v0_4::{
        NormalizedCoreContents, NormalizedGrimoire, NormalizedGrimoireMetadata,
    },
    models::v0_4::{ValidatedCoreContents, ValidatedGrimoire, ValidatedGrimoireMetadata},
    validators,
};
use std::{collections::HashMap, result as stdrt};

impl TryFrom<NormalizedGrimoire> for ValidatedGrimoire {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedGrimoire) -> stdrt::Result<Self, Self::Error> {
        Ok(Self {
            common_metadata: s.common_metadata.try_into()?,
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            core_contents: s.core_contents.try_into()?,
        })
    }
}

impl TryFrom<NormalizedGrimoireMetadata> for ValidatedGrimoireMetadata {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedGrimoireMetadata) -> stdrt::Result<Self, Self::Error> {
        let parent_object = "GrimoireMetadata";
        Ok(Self {
            common_metadata: s.common_metadata.try_into()?,
            grimoire_name: validators::ensure_str_is_not_empty(
                s.grimoire_name,
                "grimoireName",
                &parent_object,
            )?,
            grimoire_version: {
                let ver = validators::ensure_str_is_not_empty(
                    s.grimoire_version.clone(),
                    "grimoireVersion",
                    &parent_object,
                )?;
                semver::Version::parse(&ver).map_err(|e| {
                    ValidateGrimoireError::InvalidFieldValueSemverError {
                        field_name: "grimoireVersion".to_string(),
                        value: s.grimoire_version.clone(),
                        parent_object_name: parent_object.to_string(),
                        source: e,
                    }
                })?
            },
            grimoire_description: validators::ensure_some_str_is_not_empty(
                s.grimoire_description,
                "grimoireDescription",
                &parent_object,
            )?,
            grimoire_authors: validators::ensure_some_vec_is_not_empty(
                s.grimoire_authors,
                "grimoireAuthors",
                &parent_object,
            )?,
            grimoire_source_code: validators::ensure_some_str_is_not_empty(
                s.grimoire_source_code,
                "grimoireSourceCode",
                &parent_object,
            )?,
            grimoire_website: validators::ensure_some_str_is_not_empty(
                s.grimoire_website,
                "grimoireWebsite",
                &parent_object,
            )?,
            grimoire_documentation: {
                let doc = validators::ensure_some_str_is_not_empty(
                    s.grimoire_documentation,
                    "grimoireDocumentation",
                    &parent_object,
                )?;
                match doc {
                    Some(doc) if doc.ends_with('/') => {
                        return Err(ValidateGrimoireError::InvalidFieldValue {
                            field_name: "grimoireDocumentation".to_string(),
                            value: doc.clone(),
                            parent_object_name: parent_object.to_string(),
                            reason: "must not end with `/`".to_string(),
                        });
                    }
                    Some(doc) => Some(doc),
                    None => None,
                }
            },
            grimoire_readme: validators::ensure_some_str_is_not_empty(
                s.grimoire_readme,
                "grimoireReadme",
                &parent_object,
            )?,
            grimoire_license: {
                let license_str = validators::ensure_str_is_not_empty(
                    s.grimoire_license,
                    "grimoireLicense",
                    &parent_object,
                )?;

                if spdx::Expression::parse(&license_str).is_err() {
                    // Unknown license -> must have grimoire_license_text
                    if s.grimoire_license_text.is_none() {
                        return Err(ValidateGrimoireError::MissingRequiredField {
                                field_name: "grimoireLicenseText".to_string(),
                                parent_object_name: parent_object.to_string(),
                                reason: "grimoireLicenseText required when grimoireLicense is not a recognized SPDX identifier".to_string(),
                            });
                    }
                }
                license_str
            },
            grimoire_license_text: validators::ensure_some_str_is_not_empty(
                s.grimoire_license_text,
                "grimoireLicenseText",
                &parent_object,
            )?,
            grimoire_issue_tracker: validators::ensure_some_str_is_not_empty(
                s.grimoire_issue_tracker,
                "grimoireIssueTracker",
                &parent_object,
            )?,
            grimoire_keywords: validators::ensure_some_vec_is_not_empty(
                s.grimoire_keywords,
                "grimoireKeywords",
                &parent_object,
            )?,
            // Allow this field to be flexible by not validating nested fields
            grimoire_additional_metadata: validators::ensure_some_map_is_not_empty(
                s.grimoire_additional_metadata,
                "grimoireAdditionalMetadata",
                &parent_object,
            )?,
        })
    }
}

impl TryFrom<NormalizedCoreContents> for ValidatedCoreContents {
    type Error = ValidateGrimoireError;

    fn try_from(s: NormalizedCoreContents) -> stdrt::Result<Self, Self::Error> {
        let parent_object = "CoreContents";
        Ok(Self {
            grimoire_metadata: s.grimoire_metadata.try_into()?,
            chapters: validators::ensure_some_map_is_not_empty(
                s.chapters,
                "chapters",
                &parent_object,
            )?
            .map(|c| {
                c.into_iter()
                    .map(|(k, v)| {
                        let key =
                            validators::ensure_str_is_not_empty(k, "Chapter Key", &parent_object)?;
                        v.try_into().map(|nv| (key, nv))
                    })
                    .collect::<Result<HashMap<_, _>, _>>()
            })
            .transpose()?,
            rituals: validators::ensure_some_map_is_not_empty(
                s.rituals,
                "rituals",
                &parent_object,
            )?
            .map(|r| {
                r.into_iter()
                    .map(|(k, v)| {
                        let key =
                            validators::ensure_str_is_not_empty(k, "Ritual Key", &parent_object)?;
                        v.try_into().map(|nv| (key, nv))
                    })
                    .collect::<Result<HashMap<_, _>, _>>()
            })
            .transpose()?,
            auto_perform_rituals: validators::ensure_some_vec_is_not_empty(
                s.auto_perform_rituals,
                "autoPerformRituals",
                &parent_object,
            )?,
            requires_confirmation: s.requires_confirmation,
        })
    }
}
