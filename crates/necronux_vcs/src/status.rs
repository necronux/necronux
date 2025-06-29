// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use anyhow::Context;

pub fn display_repo_status() -> anyhow::Result<()> {
    #[cfg(feature = "git")]
    {
        crate::Git::display_repo_status()
    }
    #[cfg(not(feature = "git"))]
    {
        tracing::warn!(
            "No specific VCS support is enabled. Enable a VCS feature like 'git' first."
        );
        Ok(())
    }
}

pub fn is_repo_connected() -> anyhow::Result<bool> {
    let path = crate::paths::active_clone_path()?;

    #[cfg(feature = "git")]
    if crate::validate::repo_exists(&path)? {
        return Ok(true);
    }
    #[cfg(not(feature = "git"))]
    {
        tracing::warn!(
            "No specific VCS support is enabled. Enable a VCS feature like 'git' first."
        );
        return Ok(false);
    }
    Ok(false)
}

pub fn get_remote_url() -> anyhow::Result<String> {
    #[cfg(feature = "git")]
    {
        crate::Git::get_remote_url().context("Failed to get remote url")
    }
    #[cfg(not(feature = "git"))]
    {
        tracing::warn!(
            "No specific VCS support is enabled. Enable a VCS feature like 'git' first."
        );
        return Ok(String::new());
    }
}
