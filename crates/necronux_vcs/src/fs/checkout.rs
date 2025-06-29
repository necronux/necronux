// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use anyhow::Context;

pub fn checkout_revision(rev: &str) -> anyhow::Result<()> {
    #[cfg(feature = "git")]
    {
        crate::Git::checkout_revision(rev).context("Failed to checkout revision")
    }

    #[cfg(not(feature = "git"))]
    {
        tracing::warn!(
            "No specific VCS support is enabled. Enable a VCS feature like 'git' first."
        );
        return Ok();
    }
}
