// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

pub fn update_repo() -> anyhow::Result<()> {
    #[cfg(feature = "git")]
    {
        crate::Git::update_repo()
    }
    #[cfg(not(feature = "git"))]
    {
        tracing::warn!(
            "No specific VCS support is enabled. Enable a VCS feature like 'git' first."
        );
        Ok(())
    }
}
