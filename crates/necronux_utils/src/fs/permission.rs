// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::FsError;
use necronux_macros::trace_instrument;
use std::{path::Path, result as stdrt};
use tracing::debug;

#[trace_instrument(level = "debug", fields(label = %label, path = %path.display(), mode = %mode))]
pub fn set_permissions(path: &Path, label: &str, mode: u32) -> stdrt::Result<(), FsError> {
    debug!(
        label = %label,
        path = %path.display(),
        mode = %mode,
        "Attempting to set permissions...",
    );

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        match std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)) {
            Ok(_) => {
                debug!(
                    label = %label,
                    path = %path.display(),
                    mode = %mode,
                    "Successfully set permissions",
                );
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Err(FsError::SetPermissionsError {
                    label: label.to_string(),
                    path: path.to_path_buf(),
                    source: std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Path does not exist when attempting to set permissions",
                    ),
                });
            }
            Err(e) => {
                return Err(FsError::SetPermissionsError {
                    label: label.to_string(),
                    path: path.to_path_buf(),
                    source: e,
                });
            }
        }
    }

    #[cfg(not(unix))]
    {
        debug!(
            label = %label,
            path = %path.display(),
            mode = %mode,
            "Skipping permission setting as not needed or intended on non-Unix platforms",
        );
    }

    Ok(())
}
