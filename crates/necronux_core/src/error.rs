// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use std::path::PathBuf;
use thiserror::Error;

// Crate level errors

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Failed to resolve grimoire parser")]
    ResolveGrimoireParserError {
        #[source]
        source: ResolveGrimoireParserError,
    },

    #[error("Failed to validate parsed grimoire")]
    ValidateGrimoireError {
        #[source]
        source: ValidateGrimoireError,
    },
}

// Function level errors

#[derive(Debug, Error)]
pub enum ParseGrimoireError {
    #[error("Failed to parse pkl file at '{path}'")]
    ParsePklError {
        path: PathBuf,
        #[source]
        source: rpkl::Error,
    },

    #[error(transparent)]
    PkgError(#[from] necronux_pkg::error::PkgError),

    #[error(transparent)]
    PathError(#[from] necronux_utils::error::PathError),
}

#[derive(Debug, Error)]
pub enum ResolveGrimoireParserError {
    #[error("Failed to parse necronux.grimoire file")]
    ParseGrimoireError {
        #[source]
        source: ParseGrimoireError,
    },

    #[error("Unsupported or unrecognized schema version: {version}")]
    UnsupportedGrimoireSchemaVersionError { version: String },

    #[error("Expected parsed minimal metadata but not found")]
    ParsedMinimalMetadataNotFoundError,
}

#[derive(Debug, Error)]
pub enum ValidateGrimoireError {
    #[error("Missing required field in grimoire. '{field_name}' is required")]
    MissingRequiredFieldError { field_name: String },
}
