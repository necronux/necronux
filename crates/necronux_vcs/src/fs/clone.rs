// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use anyhow::{Context, anyhow};

pub fn clone_repo(url: &str) -> anyhow::Result<()> {
    #[cfg(feature = "git")]
    {
        clone_with_backup(|| crate::Git::clone_repo(url)).context("Failed to clone git repo")
    }

    #[cfg(not(feature = "git"))]
    {
        tracing::warn!(
            "No specific VCS support is enabled. Enable a VCS feature like 'git' first."
        );
        return Ok();
    }
}

pub fn clone_with_backup<F: FnOnce() -> anyhow::Result<()>>(clone_fn: F) -> anyhow::Result<()> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("clone_with_backup").entered();

    super::rename_active_clone_to_backup_active_clone()
        .context("Failed to backup active clone directory")?;

    match clone_fn() {
        Ok(_) => super::delete_backup_active_clone(),
        Err(e) => {
            super::rename_backup_active_clone_to_active_clone()
                .context("Failed to restore active clone backup after clone failure")?;
            Err(anyhow!("{e}"))
        }
    }
}
