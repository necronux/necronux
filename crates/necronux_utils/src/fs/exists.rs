// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::Result;
use crate::{error::FsError, string::capitalize_first};
use tracing::debug;

pub fn path_exists(path: &std::path::Path, label: &str) -> Result<bool> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("path_exists", label = label).entered();

    match path.try_exists() {
        Ok(true) => {
            debug!("{} exists at '{}'", capitalize_first(label), path.display());
            Ok(true)
        }
        Ok(false) => {
            debug!(
                "{} does not exist at '{}'",
                capitalize_first(label),
                path.display()
            );
            Ok(false)
        }
        Err(e) => Err(FsError::CheckPathExistsError {
            label: label.to_string(),
            path: path.to_path_buf(),
            source: e,
        }),
    }
}

pub fn file_exists(path: &std::path::Path, label: &str) -> Result<bool> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("file_exists", label = label).entered();

    match path.is_file() {
        true => {
            debug!(
                "{} exists at '{}' and is a valid file",
                capitalize_first(label),
                path.display()
            );
            Ok(true)
        }
        false => {
            debug!(
                "{} does not exist at '{}' or is not a valid file",
                capitalize_first(label),
                path.display()
            );
            Ok(false)
        }
    }
}
