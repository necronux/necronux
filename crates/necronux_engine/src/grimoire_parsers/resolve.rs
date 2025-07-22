// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{GrimoireParser, ParsedGrimoire};
use crate::error::{EngineError, Result};
use tracing::info;

pub fn resolve_grimoire_parser() -> Result<ParsedGrimoire> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("resolve_grimoire_parser").entered();

    info!("Resolving grimoire parser...");

    fn resolve_grimoire_parser_inner() -> Result<ParsedGrimoire> {
        let metadata_parsed =
            crate::grimoire_parsers::minimal_metadata::MinimalMetadataParser.parse()?;

        let grimoire_schema_major = match metadata_parsed {
            ParsedGrimoire::MinimalMetadata(m) => m.std_schema_version.major,
            #[cfg(feature = "grimoire_schema_v0")]
            ParsedGrimoire::V0(_) => {
                return Err(EngineError::ParsedMinimalMetadataNotFound {});
            }
        };

        match grimoire_schema_major {
            #[cfg(feature = "grimoire_schema_v0")]
            0 => {
                tracing::debug!(
                    "Using {} parser for schema version 0",
                    crate::V0Parser.name()
                );
                crate::V0Parser.parse()
            }

            _ => Err(EngineError::UnsupportedGrimoireSchemaVersion {
                version: grimoire_schema_major.to_string(),
            }),
        }
    }

    resolve_grimoire_parser_inner().map_err(|e| EngineError::ResolveGrimoireParserError {
        source: Box::new(e),
    })
}
