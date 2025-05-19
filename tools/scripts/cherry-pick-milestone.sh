#!/bin/bash

# ==-----------------------------------------------------------== #
# SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
#
# SPDX-License-Identifier: Apache-2.0
# ==-----------------------------------------------------------== #

read -rp "Enter the version number (e.g., 0.4.0): " version

if [ -z "$version" ]; then
    echo "Error: No version entered. Aborting."
    exit 1
fi

echo "Fetching merged PRs for milestone: $version..."
prs=$(gh pr list --repo necronux/necronux \
  --search "milestone:$version" \
  --state merged \
  --json mergeCommit,mergedAt,title,number \
  --limit 100)
if [ -z "$prs" ] || [ "$prs" = "[]" ]; then
  echo "No PRs found for milestone $version. Exiting."
  exit 1
else
  count=$(echo "$prs" | jq length)
  echo "Found $count merged PRs for milestone $version."
fi

git checkout release/v$version
echo

while read -r commit number title <&3; do
    pr_log="#$number"
    if git log --format=%s | grep -Fq "$pr_log"; then
        echo "Skipping already applied PR #$number"
        echo
        continue
    fi

    echo "Applying PR #$number: $title"
    echo
    if git cherry-pick "$commit"; then
        echo "Cherry-picked successfully."
        applied="yes"
    else
        echo "Conflict while cherry-picking PR #$number"
        echo "Please resolve the conflict, run 'git cherry-pick --continue', then press Enter to proceed to the next PR."
        read -rp "Press Enter when ready..."
        applied="manual"
    fi

    echo
    echo "Finished processing PR #$number: $title (Status: $applied)"
    echo
done 3< <(echo "$prs" | jq -r '
  sort_by(.mergedAt)[] |
  "\(.mergeCommit.oid) \(.number) \(.title)"
')
