// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use anyhow::Context;
use git2::Repository;

impl super::Git {
    pub fn update_repo() -> anyhow::Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("update_repo").entered();

        if !crate::status::is_repo_connected()
            .context("Failed to determine repo connection status")?
        {
            return Err(anyhow::anyhow!(
                "No repository connected. Use 'necronux repo connect' first"
            ));
        }

        let path = crate::paths::active_clone_path()?;
        let repo = Repository::open(&path).context("Failed to open git repository")?;

        let head = repo.head().context("Failed to get HEAD reference")?;

        if !head.is_branch() {
            return Err(anyhow::anyhow!(
                "Cannot update: currently in detached HEAD state. Use 'necronux repo connect' to checkout a branch"
            ));
        }

        let branch_name = head
            .shorthand()
            .ok_or_else(|| anyhow::anyhow!("Failed to get current branch name"))?;

        // Fetch latest changes from origin
        let mut remote = repo
            .find_remote("origin")
            .context("Failed to find origin remote")?;
        let mut fetch_opts = super::common::make_fetch_options();

        println!("Fetching latest changes from origin...");
        remote
            .fetch(
                &["+refs/heads/*:refs/remotes/origin/*"],
                Some(&mut fetch_opts),
                Some("Fetching updates"),
            )
            .context("Failed to fetch from remote")?;

        // Get the remote tracking branch
        let remote_branch_ref = format!("refs/remotes/origin/{branch_name}");
        let remote_ref = repo
            .find_reference(&remote_branch_ref)
            .with_context(|| format!("Remote branch '{branch_name}' not found"))?;

        let remote_commit = remote_ref
            .peel_to_commit()
            .context("Failed to get commit from remote reference")?;

        // Get current local commit
        let local_commit = head
            .peel_to_commit()
            .context("Failed to get current commit")?;

        if local_commit.id() == remote_commit.id() {
            println!("Already up to date.");
            return Ok(());
        }

        // Calculate how many commits behind
        let behind = repo
            .graph_ahead_behind(local_commit.id(), remote_commit.id())
            .context("Failed to calculate behind status")?
            .1; // Only care about behind count

        println!("Updating branch '{branch_name}' ({behind} commit(s) behind)...",);

        // Update the local branch to point to the remote commit
        let local_branch_ref = format!("refs/heads/{branch_name}");
        repo.reference(
            &local_branch_ref,
            remote_commit.id(),
            true,
            &format!("Update {branch_name} to {}", remote_commit.id()),
        )
        .context("Failed to update local branch reference")?;

        // Checkout the updated branch
        repo.checkout_head(Some(git2::build::CheckoutBuilder::new().force()))
            .context("Failed to checkout updated branch")?;

        println!(
            "Successfully updated to commit {}",
            remote_commit
                .id()
                .to_string()
                .chars()
                .take(7)
                .collect::<String>()
        );
        tracing::debug!(
            "Updated branch '{branch_name}' to commit {}",
            remote_commit.id()
        );

        Ok(())
    }

    pub fn is_repo_updated() -> anyhow::Result<bool> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("is_repo_updated").entered();

        if !crate::status::is_repo_connected()
            .context("Failed to determine repo connection status")?
        {
            return Err(anyhow::anyhow!(
                "No repository connected. Use 'necronux repo connect' first"
            ));
        }

        let path = crate::paths::active_clone_path()?;
        let repo = Repository::open(&path).context("Failed to open git repository.")?;

        let head = repo.head().context("Failed to get HEAD reference")?;

        // Check if in detached HEAD state
        if !head.is_branch() {
            return Err(anyhow::anyhow!(
                "Cannot check update status: currently in detached HEAD state"
            ));
        }

        let branch_name = head
            .shorthand()
            .ok_or_else(|| anyhow::anyhow!("Failed to get current branch name"))?;

        // Fetch latest changes from origin
        let mut remote = repo
            .find_remote("origin")
            .context("Failed to find origin remote")?;
        let mut fetch_opts = super::common::make_fetch_options();

        remote
            .fetch(
                &["+refs/heads/*:refs/remotes/origin/*"],
                Some(&mut fetch_opts),
                Some("Checking for updates"),
            )
            .context("Failed to fetch from remote")?;

        // Get the remote tracking branch
        let remote_branch_ref = format!("refs/remotes/origin/{branch_name}");
        let remote_ref = repo
            .find_reference(&remote_branch_ref)
            .with_context(|| format!("Remote branch '{branch_name}' not found"))?;

        let remote_commit = remote_ref
            .peel_to_commit()
            .context("Failed to get commit from remote reference")?;

        // Get current local commit
        let local_commit = head
            .peel_to_commit()
            .context("Failed to get current commit")?;

        let up_to_date = local_commit.id() == remote_commit.id();

        tracing::debug!(
            "Repo update status for branch '{branch_name}': {} (local: {}, remote: {})",
            if up_to_date { "up-to-date" } else { "behind" },
            local_commit.id(),
            remote_commit.id()
        );

        Ok(up_to_date)
    }
}
