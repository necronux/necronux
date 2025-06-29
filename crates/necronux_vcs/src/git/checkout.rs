// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use anyhow::Context;
use git2::Repository;

impl super::Git {
    pub fn checkout_revision(rev: &str) -> anyhow::Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("checkout_ref", rev = rev).entered();

        let path = crate::paths::active_clone_path()?;
        let repo = Repository::open(&path).context("Failed to open git repository")?;

        // First, ensure latest remote refs
        let mut remote = repo
            .find_remote("origin")
            .context("Failed to find origin remote")?;
        let mut fetch_opts = super::common::make_fetch_options();

        // Fetch all remote branches to ensure latest refs
        remote
            .fetch(
                &["+refs/heads/*:refs/remotes/origin/*"],
                Some(&mut fetch_opts),
                Some(&format!("Fetching branch {rev}")),
            )
            .context("Failed to fetch remote branches")?;

        // Try to find the remote branch
        let remote_branch_ref = format!("refs/remotes/origin/{rev}");
        let remote_ref_result = repo.find_reference(&remote_branch_ref);

        match remote_ref_result {
            Ok(remote_ref) => {
                // Remote branch exists, create/update local branch
                let remote_commit = remote_ref
                    .peel_to_commit()
                    .context("Failed to get commit from remote reference")?;

                let local_branch_ref = format!("refs/heads/{rev}");

                // Create or update the local branch to point to the remote commit
                repo.reference(
                    &local_branch_ref,
                    remote_commit.id(),
                    true, // force update if exists
                    &format!("Create/update local branch {rev} from remote"),
                )
                .context("Failed to create/update local branch reference")?;

                // Now get the local branch object
                let _local_branch = repo
                    .find_branch(rev, git2::BranchType::Local)
                    .context("Failed to find newly created local branch")?;

                // Set up the upstream tracking
                // We need to set the branch config manually
                let mut config = repo.config().context("Failed to get repository config")?;

                config
                    .set_str(&format!("branch.{rev}.remote"), "origin")
                    .context("Failed to set branch remote config")?;

                config
                    .set_str(&format!("branch.{rev}.merge"), &format!("refs/heads/{rev}"))
                    .context("Failed to set branch merge config")?;

                // Set HEAD to the local branch
                repo.set_head(&local_branch_ref)
                    .context("Failed to set HEAD to local branch")?;

                repo.checkout_head(Some(
                    git2::build::CheckoutBuilder::new().force(), // Force checkout to handle any conflicts
                ))
                .context("Failed to checkout HEAD")?;

                println!("Successfully checked out branch '{rev}'");
                tracing::debug!(
                    "Checked out branch '{rev}' at commit {} with upstream configured",
                    remote_commit.id()
                );

                return Ok(());
            }
            Err(_) => {
                tracing::debug!("Remote branch '{rev}' not found, trying as tag or commit");
                println!("Branch '{rev}' not found on remote, trying as tag or commit...",);
            }
        }

        // Fallback: treat reference as a tag or commit hash
        let obj = repo
            .revparse_single(rev)
            .with_context(|| format!("Failed to resolve ref '{rev}' in repository"))?;

        repo.checkout_tree(&obj, Some(git2::build::CheckoutBuilder::new().force()))
            .with_context(|| format!("Failed to checkout tree for ref '{rev}'"))?;

        // Set HEAD to detached state
        let commit = obj
            .peel_to_commit()
            .context("Failed to peel object to commit")?;
        repo.set_head_detached(commit.id())
            .with_context(|| format!("Failed to set HEAD detached at '{rev}'"))?;

        println!("Successfully checked out {rev} in detached HEAD state");
        tracing::debug!(
            "Checked out {rev} in detached HEAD state at commit {}",
            commit.id()
        );

        Ok(())
    }
}
