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

