#!/bin/bash

# ==-----------------------------------------------------------== #
# SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
#
# SPDX-License-Identifier: Apache-2.0
# ==-----------------------------------------------------------== #

# Define target branch pattern
STRICT_TARGET_PATTERN="^stable/v[0-9]+\.[0-9]+$"
LOOSE_TARGET_PATTERN="^stable/v.+\..+$"

# Define allowed source branch
ALLOWED_PATTERN="^release/v[0-9]+\.[0-9]+\.[0-9]+$"

# Check if the pull request is targeting the branch
if [[ "$GITHUB_BASE_REF" =~ $LOOSE_TARGET_PATTERN ]]; then
    if ! [[ "$GITHUB_BASE_REF" =~ $STRICT_TARGET_PATTERN ]]; then
        printf "Error: Pull request is targeting an invalid stable branch name: '%s'\n" "$GITHUB_BASE_REF"
        printf "Branch must follow SemVer format: stable/v<MAJOR>.<MINOR>\n"
        exit 1
    # Check if the source branch follows the allowed naming pattern
    elif ! [[ "$GITHUB_HEAD_REF" =~ $ALLOWED_PATTERN ]]; then
        printf "Error: Pull requests to '%s' can only be from release branches.\n" "$GITHUB_BASE_REF"
        exit 1
    else
        echo "Pull request to '$GITHUB_BASE_REF' from allowed branch '$GITHUB_HEAD_REF'."
    fi
else
    # If not targeting the branch, skip the check
    echo "Pull request is not targeting a stable branch; skipping the check."
fi

exit 0
