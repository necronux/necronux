#!/bin/bash

# ==-----------------------------------------------------------== #
# SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
#
# SPDX-License-Identifier: Apache-2.0
# ==-----------------------------------------------------------== #

# Define target branch
TARGET_BRANCH="develop"

# Define allowed branch prefixes as an array
ALLOWED_PREFIXES=("feature/" "bugfix/" "bump/" "release-sync/" "dependabot/")

# Convert array to regex pattern
ALLOWED_PATTERN="^($(IFS='|'; echo "${ALLOWED_PREFIXES[*]}"))"

# Check if the pull request is targeting the develop branch
if [[ "$GITHUB_BASE_REF" == "$TARGET_BRANCH" ]]; then
    # Check if the source branch follows the allowed naming pattern
    if ! [[ "$GITHUB_HEAD_REF" =~ $ALLOWED_PATTERN ]]; then
        printf "Error: Pull requests to '%s' can only be from these branches:\n" "$TARGET_BRANCH"
        for branch in "${ALLOWED_PREFIXES[@]}"; do
            printf "   - %s\n" "$branch"
        done
        exit 1
    else
        echo "Pull request to '$TARGET_BRANCH' from allowed branch '$GITHUB_HEAD_REF'."
    fi
else
    # If not targeting 'develop', skip the check
    echo "Pull request is not targeting '$TARGET_BRANCH'; skipping the check."
fi

exit 0
