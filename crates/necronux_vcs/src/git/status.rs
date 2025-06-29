// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use anyhow::{Context, anyhow};
use git2::Repository;
use tracing::debug;

impl super::Git {
    pub fn display_repo_status() -> anyhow::Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("display_git_repo_status").entered();

        if !crate::status::is_repo_connected()
            .context("Failed to determine repo connection status")?
        {
            println!("No connected repository found.");
            return Ok(());
        }

        let path = crate::paths::active_clone_path()?;
        let repo = Repository::open(&path).context("Failed to open git repository")?;

        let remote = repo
            .find_remote("origin")
            .context("Failed to get information for remote 'origin'")?;
        let url = remote
            .url()
            .ok_or_else(|| anyhow!("Remote 'origin' has no URL set"))?;

        let mut remote = repo
            .find_remote("origin")
            .context("Failed to find origin remote")?;
        let mut fetch_opts = super::common::make_fetch_options();

        remote
            .fetch(
                &["+refs/heads/*:refs/remotes/origin/*"],
                Some(&mut fetch_opts),
                Some("Checking status"),
            )
            .context("Failed to fetch from remote")?;

        let head = repo.head().context("Failed to get HEAD reference")?;

        // Determine if HEAD is detached
        let (branch_or_commit, is_detached) = if head.is_branch() {
            // If on branch, get branch name
            (
                head.shorthand()
                    .ok_or_else(|| anyhow!("Failed to determine current branch name"))?
                    .to_string(),
                false,
            )
        } else {
            // Detached HEAD — show short commit id
            let oid = head
                .target()
                .ok_or_else(|| anyhow!("HEAD has no target commit"))?;
            (format!("{oid}").chars().take(7).collect(), true)
        };

        // Check repo status by comparing with remote
        let status = if !is_detached {
            let branch = repo
                .find_branch(&branch_or_commit, git2::BranchType::Local)
                .context("Failed to find local branch for status check")?;

            // Try to get upstream branch
            match branch.upstream() {
                Ok(upstream) => {
                    let local_oid = branch
                        .get()
                        .target()
                        .ok_or_else(|| anyhow!("Local branch has no target commit"))?;
                    let upstream_oid = upstream
                        .get()
                        .target()
                        .ok_or_else(|| anyhow!("Upstream branch has no target commit"))?;

                    if local_oid == upstream_oid {
                        "Up to date"
                    } else {
                        // Check if local is behind remote (since no local commits expected)
                        match repo.graph_ahead_behind(local_oid, upstream_oid) {
                            Ok((ahead, behind)) => match (ahead, behind) {
                                (0, b) if b > 0 => "Behind remote (update available)",
                                (a, 0) if a > 0 => "Ahead of remote",
                                (a, b) if a > 0 && b > 0 => "Diverged from remote",
                                _ => "Up to date",
                            },
                            Err(_) => "Status unknown",
                        }
                    }
                }
                Err(_) => {
                    // No upstream configured - try to find the remote branch manually
                    let remote_ref = format!("refs/remotes/origin/{branch_or_commit}");
                    match repo.find_reference(&remote_ref) {
                        Ok(remote_branch) => {
                            let local_oid = branch
                                .get()
                                .target()
                                .ok_or_else(|| anyhow!("Local branch has no target commit"))?;
                            let remote_oid = remote_branch
                                .target()
                                .ok_or_else(|| anyhow!("Remote branch has no target commit"))?;

                            if local_oid == remote_oid {
                                "Up to date (no upstream configured)"
                            } else {
                                "Differs from remote (no upstream configured)"
                            }
                        }
                        Err(_) => "No remote branch found",
                    }
                }
            }
        } else {
            // Detached HEAD: no upstream comparison possible
            "Detached HEAD"
        };

        println!("Remote URL:     {url}");
        if is_detached {
            println!("Detached at:    {branch_or_commit}");
        } else {
            println!("Current Branch: {branch_or_commit}");
        }
        println!("Status:         {status}");

        Ok(())
    }

    pub fn get_remote_url() -> anyhow::Result<String> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("get_git_remote_url").entered();

        let path = crate::paths::active_clone_path()?;
        let repo = Repository::open(&path).context("Failed to open git repository")?;
        let remote = repo
            .find_remote("origin")
            .context("Failed to get information for remote 'origin'")?;
        let url = remote
            .url()
            .ok_or_else(|| anyhow!("Remote 'origin' has no URL set"))?;

        debug!("Retrieved URL: {:?} for repo at '{url}'", path.display());

        Ok(url.to_string())
    }
}
