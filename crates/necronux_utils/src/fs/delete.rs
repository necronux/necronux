// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::Result;
use crate::{error::FsError, string::capitalize_first};
use std::path::Path;
use tracing::{debug, warn};

pub fn remove_dir_all_if_exists(path: &Path, label: &str) -> Result<()> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("remove_dir_all", label = label).entered();

    if !super::path_exists(path, label)? {
        debug!(
            "Nothing to delete as {label} directory does not exist at '{}'",
            path.display()
        );
        return Ok(());
    }

    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("remove_dir_all_op", label = label).entered();
    match std::fs::remove_dir_all(path) {
        Ok(_) => {
            debug!(
                "Successfully deleted {label} directory at '{}'",
                path.display()
            );
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            warn!(
                "{} was expected to exist but not found at '{}'",
                capitalize_first(label),
                path.display()
            );
            Ok(())
        }
        Err(e) => Err(FsError::RemoveDirError {
            label: label.to_string(),
            path: path.to_path_buf(),
            source: e,
        }),
    }
}
