#!/bin/bash

# ==-----------------------------------------------------------== #
# SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
#
# SPDX-License-Identifier: Apache-2.0
# ==-----------------------------------------------------------== #

# Define target branch pattern
TARGET_PATTERN="^release/v[0-9]+\.[0-9]+\.x$"

# Define allowed source branch prefixes
ALLOWED_PREFIXES=("bump/" "feature/" "bugfix/" "hotfix/")

# Convert array to regex pattern
ALLOWED_PATTERN="^($(IFS='|'; echo "${ALLOWED_PREFIXES[*]}"))"

# Check if the pull request is targeting the release branch
if [[ "$GITHUB_BASE_REF" =~ $TARGET_PATTERN ]]; then
    # Check if the source branch follows the allowed naming pattern
    if ! [[ "$GITHUB_HEAD_REF" =~ $ALLOWED_PATTERN ]]; then
        printf "Error: Pull requests to '%s' branches can only be from these branches:\n" "$GITHUB_BASE_REF"
        for branch in "${ALLOWED_PREFIXES[@]}"; do
            printf "   - %s\n" "$branch"
        done
        exit 1
    else
        echo "Pull request to '$GITHUB_BASE_REF' from allowed branch '$GITHUB_HEAD_REF'."
    fi
else
    # If not targeting a release branch, skip the check
    echo "Pull request is not targeting a release branch; skipping the check."
fi

exit 0

