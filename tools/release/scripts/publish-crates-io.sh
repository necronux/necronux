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

if ! command -v reuse >/dev/null 2>&1; then
    echo "Error: 'reuse' is not installed. Please install it first."
    echo "More info: https://github.com/fsfe/reuse-tool"
    exit 1
fi

OUTPUT_FILE="docs/reuse.spdx"
echo "Generating SBOM at: $OUTPUT_FILE..."

if reuse spdx --output "$OUTPUT_FILE"; then
    echo "SBOM generated successfully: $OUTPUT_FILE"
else
    echo "Error: Failed to generate SBOM."
    exit 1
fi

pushd crates || { echo "Error: Failed to navigate to crates directory."; exit 1; }

for crate in $(cargo package --workspace 2>&1 | grep Packaging | sed 's_.*crates/\(.*\))_\1_' | grep -v Packaging); do
    echo "Publishing crate: $crate..."

    pushd "$crate" || { echo "Error: Failed to navigate into $crate"; exit 1; }

    if cargo publish --allow-dirty; then
        echo "Successfully published: $crate"
    else
        echo "Error: Failed to publish: $crate"
        exit 1
    fi

    popd || { echo "Error: Failed to return from $crate"; exit 1; }

    echo "Waiting 20 seconds before publishing the next crate..."
    sleep 20
done

popd || { echo "Error: Failed to return from crates directory."; exit 1; }

echo "Publishing the root crate..."
if cargo publish --allow-dirty; then
    echo "Successfully published the root crate."
else
    echo "Error: Failed to publish the root crate."
    exit 1
fi
