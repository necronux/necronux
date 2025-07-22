// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::Result;
use crate::error::FsError;
use std::path::Path;
use tracing::debug;

pub fn write(path: &Path, label: &str, content: String) -> Result<()> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("write", label = label).entered();

    std::fs::write(path, content).map_err(|e| FsError::WriteFileError {
        label: label.to_string(),
        path: path.to_path_buf(),
        source: e,
    })?;
    debug!("Successfully wrote {label} at '{}'", path.display());
    Ok(())
}
