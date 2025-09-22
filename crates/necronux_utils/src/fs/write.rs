// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::FsError;
use necronux_macros::trace_instrument;
use std::{path::Path, result as stdrt};
use tracing::debug;

#[trace_instrument(level = "debug", skip(content), fields(label = %label, path = %path.display()))]
pub fn write(path: &Path, label: &str, content: String) -> stdrt::Result<(), FsError> {
    debug!("Attempting to write {label} at '{}'", path.display());

    std::fs::write(path, content).map_err(|e| FsError::WriteFileError {
        label: label.to_string(),
        path: path.to_path_buf(),
        source: e,
    })?;

    debug!("Successfully wrote {label} at '{}'", path.display());
    Ok(())
}
