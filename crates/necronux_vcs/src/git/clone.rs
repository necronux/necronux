// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use anyhow::Context;

impl super::Git {
    pub fn clone_repo(url: &str) -> anyhow::Result<()> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("git_clone_repo", url = url).entered();

        let path = crate::paths::active_clone_path()?;

        // Clean up any existing clone debris
        necronux_utils::fs::remove_dir_all(&path, "active clone directory")?;

        let fetch_opts = super::common::make_fetch_options();

        // Configure repository builder
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_opts);

        println!("Cloning repository from: {url}");

        // Clone repository
        let repo = builder.clone(url, &path).with_context(|| {
            format!(
                "Failed to clone git repo from url: '{url}' at '{}'",
                path.display()
            )
        })?;

        // Ensure all remote branches and tags are available
        let mut remote = repo
            .find_remote("origin")
            .context("Failed to find origin remote after clone")?;
        let mut fetch_opts = super::common::make_fetch_options();

        // First, get the default remote HEAD to know which branch to track
        remote.connect(git2::Direction::Fetch)?;
        let default_branch = remote
            .default_branch()
            .map(|s| {
                s.as_str()
                    .and_then(|s| s.strip_prefix("refs/heads/"))
                    .unwrap_or("main")
                    .to_string()
            })
            .unwrap_or_else(|_| "main".to_string());
        remote
            .disconnect()
            .context("Failed to disconnect from remote")?;

        // Fetch all branches and tags with explicit refspecs
        let refspecs = [
            "+refs/heads/*:refs/remotes/origin/*",
            "+refs/tags/*:refs/tags/*",
        ];

        remote
            .fetch(&refspecs, Some(&mut fetch_opts), Some("Fetching all refs"))
            .context("Failed to fetch all remote refs after clone")?;

        println!("Successfully cloned repository from: {url}");
        tracing::debug!(
            "Repository cloned from url: '{url}' and initialized at '{}', default branch: '{default_branch}'",
            path.display(),
        );

        Ok(())
    }
}
