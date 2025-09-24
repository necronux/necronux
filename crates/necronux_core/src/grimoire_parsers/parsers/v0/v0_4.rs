// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::v0_common;
use crate::{GrimoireParser, error::ParseGrimoireError, grimoire_schemas::ParsedGrimoire};
use std::result as stdrt;

#[derive(Debug)]
pub struct V0_4Parser;

impl GrimoireParser for V0_4Parser {
    fn name(&self) -> &str {
        "v0.4 parser"
    }

    fn parse_inner(&self) -> stdrt::Result<ParsedGrimoire, ParseGrimoireError> {
        let grimoire_file = v0_common::parse_inner_common()?;
        let parsed =
            rpkl::from_config(&grimoire_file).map_err(|e| ParseGrimoireError::RpklError {
                path: grimoire_file.to_path_buf(),
                source: e,
            })?;
        Ok(ParsedGrimoire::V0_4(parsed))
    }
}
