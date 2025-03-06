#!/bin/bash

# ==-----------------------------------------------------------== #
# SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
#
# SPDX-License-Identifier: Apache-2.0
# ==-----------------------------------------------------------== #

# Check if the target branch is 'stable'
if [[ "${GITHUB_BASE_REF}" == "stable" ]]; then
    # If targeting 'stable', ensure source branch is 'release/*' only
    if ! [[ ${GITHUB_HEAD_REF} =~ ^release/.+ ]]; then
        echo "Error: Pull requests to stable can only be from 'release/*' branches."
        exit 1
    else
        echo "Pull request to stable from allowed branch '${GITHUB_HEAD_REF}'."
    fi
else
    # If not targeting 'stable', skip the check and pass
    echo "Pull request is not targeting 'stable'; no protection checks required."
fi
