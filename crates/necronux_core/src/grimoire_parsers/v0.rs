// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{GrimoireParser, ParsedGrimoire};
use crate::error::ParseGrimoireError;
use necronux_utils::trace_instrument;
use std::result as stdrt;
use tracing::debug;

#[derive(Debug)]
pub struct V0Parser;

impl GrimoireParser for V0Parser {
    fn name(&self) -> &str {
        "v0"
    }

    #[trace_instrument(level = "debug", name = "V0Parser::parse", skip(self))]
    fn parse(&self) -> stdrt::Result<ParsedGrimoire, ParseGrimoireError> {
        debug!(
            parser = %self.name(),
            "Parsing grimoire with {} parser...", self.name()
        );

        fn parse_inner() -> stdrt::Result<ParsedGrimoire, ParseGrimoireError> {
            let current_grimoire_dir = necronux_utils::paths::current_grimoire_path()?;
            let grimoire_info = necronux_pkg::CurrentGrimoireInfo::introspect()?;
            let grimoire_file = current_grimoire_dir
                .join(grimoire_info.package_name)
                .join("necronux.grimoire");

            let parsed = rpkl::from_config(&grimoire_file).map_err(|e| {
                ParseGrimoireError::ParsePklError {
                    path: grimoire_file.to_path_buf(),
                    source: e,
                }
            })?;
            Ok(ParsedGrimoire::V0(Box::new(parsed)))
        }

        let parsed = parse_inner()?;

        debug!(
            parser = %self.name(),
            "Successfully parsed grimoire with {} parser", self.name()
        );
        Ok(parsed)
    }
}
