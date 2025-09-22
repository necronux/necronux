// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{CommonMetadataParser, GrimoireParser};
use crate::{
    TryIntoValidatedGrimoire, V0Parser, ValidatedGrimoire,
    error::{ResolveGrimoireParserError, ResolveGrimoireParserErrorWithContext},
    grimoire_normalized_models::TryIntoNormalizedGrimoire,
    grimoire_schemas::ParsedGrimoire,
};
#[cfg(feature = "grimoire_schema_v0")]
use necronux_utils::trace_instrument;
use std::result as stdrt;
use tracing::debug;

#[trace_instrument(level = "debug")]
pub fn resolve_grimoire_parser()
-> stdrt::Result<ParsedGrimoire, ResolveGrimoireParserErrorWithContext> {
    debug!("Resolving grimoire parser...");

    let resolved = resolve_grimoire_parser_inner()
        .map_err(|e| ResolveGrimoireParserErrorWithContext { source: e })?;

    debug!("Successfully resolved grimoire parser");
    Ok(resolved)
}

fn resolve_grimoire_parser_inner() -> stdrt::Result<ParsedGrimoire, ResolveGrimoireParserError> {
    let common_metadata_validated = CommonMetadataParser
        .parse()?
        .try_into_normalized()?
        .try_into_validated()?;

    let grimoire_schema_major = match common_metadata_validated {
        ValidatedGrimoire::CommonMetadata(c) => c.schema_version_info.schema_version.major,
        #[cfg(feature = "grimoire_schema_v0")]
        ValidatedGrimoire::V0(_) => {
            return Err(ResolveGrimoireParserError::ParsedCommonMetadataNotFound);
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
            Ok(parser.parse()?)
        }

        _ => Err(
            ResolveGrimoireParserError::UnsupportedGrimoireSchemaVersion {
                version: grimoire_schema_major.to_string(),
            },
        ),
    }
}
