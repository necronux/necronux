// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{CommonMetadataParser, GrimoireParser};
#[cfg(feature = "grimoire_schema_v0")]
use crate::V0_4Parser;
use crate::{
    TryIntoValidatedGrimoire, ValidatedGrimoire,
    error::{ResolveGrimoireParserError, ResolveGrimoireParserErrorWithContext},
    grimoire_normalized_models::TryIntoNormalizedGrimoire,
    grimoire_schemas::ParsedGrimoire,
};
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

    let (s_ver, s_major, s_minor) = match common_metadata_validated {
        ValidatedGrimoire::CommonMetadata(c) => {
            let ver = c.schema_version_info.schema_version;
            (ver.clone(), ver.major, ver.minor)
        }
        #[cfg(feature = "grimoire_schema_v0")]
        ValidatedGrimoire::V0_4(_) => {
            return Err(ResolveGrimoireParserError::ParsedCommonMetadataNotFound);
        }
    };

    match (s_major, s_minor) {
        #[cfg(feature = "grimoire_schema_v0")]
        (0, 4) => {
            let parser = V0_4Parser;
            let parser_name = parser.name();
            debug!(
                parser_name = %parser_name,
                "Using {} for schema version 0.4", parser_name
            );
            Ok(parser.parse()?)
        }

        _ => Err(
            ResolveGrimoireParserError::UnsupportedGrimoireSchemaVersion {
                version: s_ver.to_string(),
            },
        ),
    }
}
