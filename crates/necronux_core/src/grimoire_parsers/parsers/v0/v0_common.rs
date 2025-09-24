// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::ParseGrimoireError;
use std::{path::PathBuf, result as stdrt};

pub fn parse_inner_common() -> stdrt::Result<PathBuf, ParseGrimoireError> {
    let current_grimoire_dir = necronux_utils::paths::current_grimoire_path()?;
    let grimoire_info = necronux_pkg::CurrentGrimoireInfo::introspect()?;
    Ok(current_grimoire_dir
        .join(grimoire_info.package_name)
        .join("necronux.grimoire"))
}
