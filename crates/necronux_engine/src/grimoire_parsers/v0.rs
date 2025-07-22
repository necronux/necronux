// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{GrimoireParser, ParsedGrimoire};
use crate::error::{EngineError, Result};
use tracing::{debug, info};

pub struct V0Parser;

impl GrimoireParser for V0Parser {
    fn name(&self) -> &str {
        "v0"
    }

    fn parse(&self) -> Result<ParsedGrimoire> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("parse_grimoire_v0").entered();

        info!("Parsing grimoire with {} parser...", self.name());

        fn parse_inner(parser: &V0Parser) -> Result<ParsedGrimoire> {
            let current_grimoire_dir = necronux_utils::paths::current_grimoire_path()?;
            let grimoire_info = necronux_pkg::CurrentGrimoireInfo::introspect()?;
            let grimoire_file = current_grimoire_dir
                .join(grimoire_info.package_name)
                .join("necronux.grimoire");

            let parsed: crate::grimoire_schemas::v0::Grimoire =
                rpkl::from_config(&grimoire_file).map_err(|e| EngineError::ParsePklError {
                    path: grimoire_file.to_path_buf(),
                    source: e,
                })?;
            debug!(
                "Successfully parsed grimoire with {} parser at '{}'",
                parser.name(),
                &grimoire_file.display()
            );
            Ok(ParsedGrimoire::V0(Box::new(parsed)))
        }

        parse_inner(self).map_err(|e| EngineError::ParseGrimoireError {
            source: Box::new(e),
        })
    }
}
