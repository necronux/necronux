<!--
# ==-----------------------------------------------------------== #
# SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
#
# SPDX-License-Identifier: CC-BY-SA-4.0
# ==-----------------------------------------------------------== #
-->

### Release Process

Necronux follows a flexible release approach, prioritizing stability over strict schedules.

Instead of adhering to a fixed release cycle, new versions are released when they are ready. This
ensures that every update is well-tested and polished before reaching users. By avoiding rigid
deadlines, we maintain a sustainable development pace while delivering high-quality improvements.

## Single Versioning Policy

Necronux follows a **single version across all primary products** in the monorepo to maintain simplicity
and consistency. For a deeper understanding of how branches are managed throughout the release process,
please refer to the [Branching and Tagging Strategy](docs/branching-and-tagging-strategy.md) before proceeding with the release process.

## Development Cycles

During each development cycle, important pull requests and issues are tracked using a GitHub Milestone.
Items in the milestone are given priority, with a focus on resolving as many as possible before
the next release.

As a release approaches, the milestone is reviewed, and anything unlikely to be ready in time is
deferred to a future cycle. Once the milestone is complete, a release candidate (e.g., 1.x.0-rc.1)
is published, followed by final testing and release preparations.

## Release Candidates

As a release approaches, Necronux publishes a release candidate (e.g., 1.x.0-rc.1) to allow for
final testing and stability checks. Release candidates are typically made available about a week
before the final release.

This process helps identify any last-minute bugs or regressions and ensures that everything is
polished before the full release. If critical issues are found, they are fixed, and a new release
candidate is published. At this point, either:

- A new target release date is set if more testing time is needed, or
- The existing target date is kept if the issues are minor and can be addressed in time.

Once a release candidate meets expectations and no major issues remain, the final release is published.

## Release Checklist

When making a release, the Maintainers follow these checklists:

### Minor Release Candidate

#### Minor Pre-release (not to be confused with unstable release)

1. Check appropriate milestone.
2. Create appropriate release branch (if not already) from latest `develop` branch as `release/vX.Y.x` where X and Y are Major and Minor versions respectively.
3. In develop branch, bump version number of all components to next planned version with `-dev` suffix using `bash tools/release/scripts/bump.sh`.
4. In release branch, bump version number of all components to next minor version with `-rc.N` suffix using `bash tools/release/scripts/bump.sh`.
  - Change the commit message to be nicer: git commit --amend -m "Release X.Y.Z-rc.N"
5. Create a tag on GitHub.

For subsuquent release candidates, follow the order: `1. -> 4. -> 5.`

#### Minor Release

1. Create release using release workflow.
2. Edit GitHub Release.
3. Release on crates.io using `bash tools/release/scripts/publish-crates-io.sh`

#### Minor Post-release

1. Announce on community channels.

### Minor Stable Version

#### Minor Pre-release (not to be confused with unstable release)

1. Check regressions label.
2. Check appropriate milestone.
3. Close the milestone, open the next one if anything remains and transfer them.
4. In release branch, bump version number of all components to next minor version using `bash tools/release/scripts/bump.sh`.
  - Change the commit message to be nicer: git commit --amend -m "Release X.Y.Z"
5. Create a tag on GitHub.
6. Bump `latest` tag to most recent stable release.

#### Minor Release

1. Create release using release workflow.
2. Edit GitHub Release.
3. Release on crates.io using `bash tools/release/scripts/publish-crates-io.sh`

#### Minor Post-release

1. Announce on community channels.

### Patch Stable Version

#### Patch Pre-release (not to be confused with unstable release)

1. Check appropriate milestone.
2. Create appropriate release branch (if not already) from latest `develop` branch as `release/vX.Y.x` where X and Y are Major and Minor versions respectively.
3. Close the milestone, open the next one if anything remains and transfer them.
4. In release branch, bump version number of all components to next patch version using `bash tools/release/scripts/bump.sh`.
  - Change the commit message to be nicer: git commit --amend -m "Release X.Y.Z"
5. Create a tag on GitHub.
6. Bump `latest` tag to most recent stable release.

#### Patch Release

1. Create release using release workflow.
2. Edit GitHub Release.
3. Release on crates.io using `bash tools/release/scripts/publish-crates-io.sh`

#### Patch Post-release

1. Announce on community channels.

### Major Release Candidate

#### Major Pre-release (not to be confused with unstable release)

1. Check appropriate milestone.
2. Create appropriate release branch (if not already) from latest `develop` branch as `release/vX.Y.x` where X and Y are Major and Minor versions respectively.
3. In develop branch, bump version number of all components to next planned version with `-dev` suffix using `bash tools/release/scripts/bump.sh`.
4. In release branch, bump version number of all components to next major version with `-rc.N` suffix using `bash tools/release/scripts/bump.sh`.
  - Change the commit message to be nicer: git commit --amend -m "Release X.Y.Z-rc.N"
5. Create a tag on GitHub.

For subsuquent release candidates, follow the order: `1. -> 4. -> 5.`

#### Major Release

1. Create release using release workflow.
2. Edit GitHub Release.
3. Release on crates.io using `bash tools/release/scripts/publish-crates-io.sh`

#### Major Post-release

1. Announce on community channels.

### Major Stable Version

#### Major Pre-release (not to be confused with unstable release)

1. Check regressions label.
2. Check appropriate milestone.
3. Close the milestone, open the next one if anything remains and transfer them.
4. In release branch, bump version number of all components to next major version using `bash tools/release/scripts/bump.sh`.
  - Change the commit message to be nicer: git commit --amend -m "Release X.Y.Z"
5. Create a tag on GitHub.
6. Bump `latest` tag to most recent stable release.

#### Major Release

1. Create release using release workflow.
2. Edit GitHub Release.
3. Release on crates.io using `bash tools/release/scripts/publish-crates-io.sh`

#### Major Post-release

1. Announce on community channels.
