// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Failed to parse grimoire")]
pub struct ParseGrimoireErrorWithContext {
    #[source]
    pub source: ParseGrimoireError,
}
#[derive(Debug, Error)]
pub enum ParseGrimoireError {
    #[error("Failed to parse pkl file at '{path}'")]
    RpklError {
        path: PathBuf,
        #[source]
        source: rpkl::Error,
    },
    #[error(transparent)]
    IntrospectCurrentGrimoireError(
        #[from] necronux_pkg::error::IntrospectCurrentGrimoireErrorWithContext,
    ),
    #[error(transparent)]
    PathError(#[from] necronux_utils::error::PathError),
}

#[derive(Debug, Error)]
#[error("Failed to normalize grimoire")]
pub struct NormalizeGrimoireErrorWithContext {
    #[source]
    pub source: NormalizeGrimoireError,
}
#[derive(Debug, Error)]
pub enum NormalizeGrimoireError {
    #[error("Missing required field in grimoire: '{field_name}'")]
    MissingTopLevelRequiredField { field_name: String },
    #[error("Missing required field in grimoire: '{field_name}' (in {parent_object_name})")]
    MissingRequiredField {
        field_name: String,
        parent_object_name: String,
    },
    #[error(transparent)]
    SemverError(#[from] semver::Error),
}

#[derive(Debug, Error)]
#[error("Failed to validate grimoire")]
pub struct ValidateGrimoireErrorWithContext {
    #[source]
    pub source: ValidateGrimoireError,
}
#[derive(Debug, Error)]
pub enum ValidateGrimoireError {}

#[derive(Debug, Error)]
#[error("Failed to resolve grimoire parser")]
pub struct ResolveGrimoireParserErrorWithContext {
    #[source]
    pub source: ResolveGrimoireParserError,
}
#[derive(Debug, Error)]
pub enum ResolveGrimoireParserError {
    #[error("Expected parsed common metadata but not found")]
    ParsedCommonMetadataNotFound,
    #[error("Unsupported or unrecognized schema version: {version}")]
    UnsupportedGrimoireSchemaVersion { version: String },
    #[error(transparent)]
    ParseGrimoireError(#[from] ParseGrimoireErrorWithContext),
    #[error(transparent)]
    NormalizeGrimoireError(#[from] NormalizeGrimoireErrorWithContext),
    #[error(transparent)]
    ValidateGrimoireError(#[from] ValidateGrimoireErrorWithContext),
}
