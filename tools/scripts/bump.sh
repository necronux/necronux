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

echo "Version bump complete!"
