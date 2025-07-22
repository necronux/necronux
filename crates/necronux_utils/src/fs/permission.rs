// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::Result;
use crate::error::FsError;
use std::path::Path;

pub fn set_permissions_if_exists(path: &Path, label: &str, mode: u32) -> Result<()> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("set_permissions", label = label, mode = mode).entered();

    if !super::path_exists(path, label)? {
        return Err(FsError::SetPermissionsError {
            label: label.to_string(),
            path: path.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Path does not exist when attempting to set permissions",
            ),
        });
    }

    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("set_permissions_op", label = label, mode = mode).entered();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).map_err(|e| {
            FsError::SetPermissionsError {
                label: label.to_string(),
                path: path.to_path_buf(),
                source: e,
            }
        })?;

        tracing::debug!(
            "Successfully set permissions (mode: {mode}) on {label} at '{}'",
            path.display()
        );
    }
    Ok(())
}
