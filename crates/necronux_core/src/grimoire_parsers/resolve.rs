// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{GrimoireParser, ParsedGrimoire};
#[cfg(feature = "grimoire_schema_v0")]
use crate::V0Parser;
use crate::error::{CoreError, ResolveGrimoireParserError as ResGrimParserError};
use crate::grimoire_parsers::minimal_metadata as mm;
use necronux_utils::trace_instrument;
use std::result as stdrt;
use tracing::debug;

#[trace_instrument(level = "debug")]
pub fn resolve_grimoire_parser() -> stdrt::Result<ParsedGrimoire, CoreError> {
    debug!("Resolving grimoire parser...");

    fn resolve_grimoire_parser_inner() -> stdrt::Result<ParsedGrimoire, ResGrimParserError> {
        let metadata_parsed = mm::MinimalMetadataParser
            .parse()
            .map_err(|e| ResGrimParserError::ParseGrimoireError { source: e })?;

        let grimoire_schema_major = match metadata_parsed {
            ParsedGrimoire::MinimalMetadata(m) => m.std_schema_version.major,
            #[cfg(feature = "grimoire_schema_v0")]
            ParsedGrimoire::V0(_) => {
                return Err(ResGrimParserError::ParsedMinimalMetadataNotFoundError);
            }
        };

        match grimoire_schema_major {
            #[cfg(feature = "grimoire_schema_v0")]
            0 => {
                let parser = V0Parser;
                debug!(
                    parser = %parser.name(),
                    "Using {} parser for schema version 0", parser.name()
                );
                parser
                    .parse()
                    .map_err(|e| ResGrimParserError::ParseGrimoireError { source: e })
            }

            _ => Err(ResGrimParserError::UnsupportedGrimoireSchemaVersionError {
                version: grimoire_schema_major.to_string(),
            }),
        }
    }

    let resolved = resolve_grimoire_parser_inner()
        .map_err(|e| CoreError::ResolveGrimoireParserError { source: e })?;

    debug!("Successfully resolved grimoire parser");
    Ok(resolved)
}
