// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{error::ParseGrimoireError, grimoire_parsed_models::ParsedGrimoire, GrimoireParser};
use std::result as stdrt;

#[derive(Debug)]
pub struct CommonMetadataParser;

impl GrimoireParser for CommonMetadataParser {
    fn name(&self) -> &str {
        "common metadata parser"
    }

    fn parse_inner(&self) -> stdrt::Result<ParsedGrimoire, ParseGrimoireError> {
        let current_grimoire_dir = necronux_utils::paths::current_grimoire_path()?;
        let grimoire_info = necronux_pkg::CurrentGrimoireInfo::introspect()?;
        let grimoire_file = current_grimoire_dir
            .join(grimoire_info.package_name)
            .join("necronux.grimoire");

        let parsed =
            rpkl::from_config(&grimoire_file).map_err(|e| ParseGrimoireError::RpklError {
                path: grimoire_file.to_path_buf(),
                source: e,
            })?;
        Ok(ParsedGrimoire::CommonMetadata(parsed))
    }
}
