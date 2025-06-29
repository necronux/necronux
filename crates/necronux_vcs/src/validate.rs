// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use anyhow::anyhow;
use git2::Repository;
use std::path::Path;
use tracing::debug;

pub fn repo_exists(path: &Path) -> anyhow::Result<bool> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("git_repo_exists").entered();

    #[cfg(feature = "git")]
    {
        match Repository::open(path) {
            Ok(_) => {
                debug!("Git repo exists at '{}'", path.display());
                Ok(true)
            }
            Err(err) if err.code() == git2::ErrorCode::NotFound => {
                debug!("Git repo not found at '{}'", path.display());
                Ok(false)
            }
            Err(err) => Err(anyhow!(
                "Failed to check Git repository: {} at '{}'",
                err,
                path.display()
            )),
        }
    }

    #[cfg(not(feature = "git"))]
    {
        tracing::warn!(
            "No specific VCS support is enabled. Enable a VCS feature like 'git' first."
        );
        return Ok(false);
    }
}
