#!/bin/bash

# ==-----------------------------------------------------------== #
# SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
#
# SPDX-License-Identifier: Apache-2.0
# ==-----------------------------------------------------------== #

if [ -n "$(git status --porcelain)" ]; then
    echo "Error: You have uncommitted changes. Please commit or stash them before continuing."
    exit 1
fi

if ! command -v cargo-set-version >/dev/null 2>&1; then
    echo "Error: 'cargo-set-version' is not installed."
    echo "Install it with: cargo install cargo-set-version"
    exit 1
fi

read -rp "Enter the new version number (e.g., 0.4.0): " NEW_VERSION

if [ -z "$NEW_VERSION" ]; then
    echo "Error: No version entered. Aborting."
    exit 1
fi

if cargo set-version "$NEW_VERSION"; then
    echo "Successfully updated all crates to version: $NEW_VERSION"
else
    echo "Error: Failed to update crate versions."
    exit 1
fi

if cargo update; then
    echo "Successfully updated Cargo.lock"
else
    echo "Error: Failed to update Cargo.lock"
    exit 1
fi

PKL_FILES=(
    "stdschema/PklProject"
    "tools/necronux.internal.ci/PklProject"
)

for PKL_FILE in "${PKL_FILES[@]}"; do
    if [ -f "$PKL_FILE" ]; then
        sed -i "s/version = \".*\"/version = \"$NEW_VERSION\"/" "$PKL_FILE"
        echo "Successfully updated $PKL_FILE to version: $NEW_VERSION"
    else
        echo "Warning: $PKL_FILE not found. Skipping."
    fi
done

echo "Version bump complete!"
