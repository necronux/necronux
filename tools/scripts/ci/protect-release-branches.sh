#!/bin/bash

# ==-----------------------------------------------------------== #
# SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
#
# SPDX-License-Identifier: Apache-2.0
# ==-----------------------------------------------------------== #

# Define target branch pattern
STRICT_TARGET_PATTERN="^release/v[0-9]+\.[0-9]+\.[0-9]+$"
LOOSE_TARGET_PATTERN="^release/v.+\..+\..+$"

# Check if the pull request is targeting the branch
if [[ "$GITHUB_BASE_REF" =~ $LOOSE_TARGET_PATTERN ]]; then
    if ! [[ "$GITHUB_BASE_REF" =~ $STRICT_TARGET_PATTERN ]]; then
        printf "Error: Pull request is targeting an invalid release branch name: '%s'\n" "$GITHUB_BASE_REF"
        printf "Branch must follow SemVer format: release/v<MAJOR>.<MINOR>.<PATCH>\n"
        exit 1
    else
        printf "Error: Pull requests to '%s' branches are not allowed.\n" "$GITHUB_BASE_REF"
        exit 1
    fi
else
    # If not targeting the branch, skip the check
    echo "Pull request is not targeting a release branch; skipping the check."
fi

exit 0
