// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{
    error::{ParseGrimoireError, ParseGrimoireErrorWithContext},
    grimoire_schemas::ParsedGrimoire,
};
use necronux_utils::trace_instrument;
use std::result as stdrt;
use tracing::debug;

pub trait GrimoireParser {
    fn name(&self) -> &str;

    #[trace_instrument(level = "debug", name = "parse_grimoire", skip(self))]
    fn parse(&self) -> stdrt::Result<ParsedGrimoire, ParseGrimoireErrorWithContext> {
        let parser_name = self.name();

        debug!(
            parser = %parser_name,
            "Parsing grimoire with {parser_name}..."
        );

        let parsed = self
            .parse_inner()
            .map_err(|e| ParseGrimoireErrorWithContext { source: e })?;

        debug!("Successfully parsed grimoire with {parser_name}");
        Ok(parsed)
    }

    fn parse_inner(&self) -> stdrt::Result<ParsedGrimoire, ParseGrimoireError>;
}
