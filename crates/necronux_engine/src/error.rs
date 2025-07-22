// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, EngineError>;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Failed to parse pkl file at '{path}'")]
    ParsePklError {
        path: PathBuf,
        #[source]
        source: rpkl::Error,
    },

    #[error("Failed to parse necronux.grimoire file")]
    ParseGrimoireError {
        #[source]
        source: Box<EngineError>,
    },

    #[error("Failed to resolve grimoire parser")]
    ResolveGrimoireParserError {
        #[source]
        source: Box<EngineError>,
    },

    #[error("Failed to validate parsed grimoire")]
    ValidateGrimoireError {
        #[source]
        source: Box<EngineError>,
    },

    #[error("Failed to unify parsed grimoire")]
    UnifyGrimoireError {
        #[source]
        source: Box<EngineError>,
    },

    #[error("Missing required field in grimoire. '{field_name}' is required")]
    MissingRequiredField { field_name: String },

    #[error("Failed to find grimoire schema version at '{path}'")]
    GrimoireSchemaVersionNotFound { path: PathBuf },

    #[error("Unsupported or unrecognized schema version: {version}")]
    UnsupportedGrimoireSchemaVersion { version: String },

    #[error("Expected parsed minimal metadata but not found")]
    ParsedMinimalMetadataNotFound {},

    #[error(transparent)]
    Pkg(#[from] necronux_pkg::error::PkgError),

    #[error(transparent)]
    Path(#[from] necronux_utils::error::PathError),

    #[error(transparent)]
    Fs(#[from] necronux_utils::error::FsError),
}
