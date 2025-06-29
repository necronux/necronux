// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use anyhow::{Context, anyhow};
use std::path::Path;
use tracing::{debug, warn};

pub fn remove_dir_all(path: &Path, label: &str) -> anyhow::Result<()> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("remove_dir_all").entered();

    if crate::paths::path_exists(path, label)? {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("remove_dir_op", label = label).entered();

        match std::fs::remove_dir_all(path) {
            Ok(_) => {
                debug!("Successfully deleted {} at '{}'", label, path.display());
                Ok(())
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                warn!(
                    "{} was expected to exist but not found at '{}'",
                    label,
                    path.display()
                );
                Ok(())
            }
            Err(err) => Err(anyhow!(
                "Failed to delete {} at '{}': {}",
                label,
                path.display(),
                err
            )),
        }
    } else {
        debug!(
            "Nothing to delete as {} does not exist at '{}'",
            label,
            path.display()
        );
        Ok(())
    }
}

pub fn rename_dir(old_path: &Path, label: &str, new_path: &Path) -> anyhow::Result<()> {
    if crate::paths::path_exists(old_path, label)? {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("rename_dir_op").entered();

        std::fs::rename(old_path, new_path).with_context(|| {
            format!(
                "Failed to rename '{}' to '{}'",
                old_path.display(),
                new_path.display()
            )
        })?;

        debug!(
            "Successfully renamed '{}' to '{}'",
            old_path.display(),
            new_path.display()
        );
    } else {
        debug!(
            "Nothing to rename as {} does not exist at '{}'",
            label,
            old_path.display()
        );
    }

    Ok(())
}
