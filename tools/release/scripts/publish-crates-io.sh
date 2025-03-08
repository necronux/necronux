#!/bin/bash

# ==-----------------------------------------------------------== #
# SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
#
# SPDX-License-Identifier: Apache-2.0
# ==-----------------------------------------------------------== #

if [ -n "$(git status --porcelain)" ]; then
    echo "You have local changes!"
    exit 0
fi

if ! reuse --version &> /dev/null
then
    echo "Error: reuse is not installed. Install it first."
    exit 1
fi

OUTPUT_FILE="docs/reuse.spdx"

if reuse spdx --output "$OUTPUT_FILE"; then
    echo "SBOM generated successfully: $OUTPUT_FILE"
else
    echo "Error: Failed to generate SBOM"
    exit 1
fi

pushd crates || exit

for crate in `cargo package --workspace 2>&1 | grep Packaging | sed 's_.*crates/\(.*\))_\1_' | grep -v Packaging`
do
  echo "Publishing ${crate}"
  pushd "$crate" || exit
  cargo publish
  popd || exit
  sleep 20
done

popd || exit

echo "Publishing root crate"
cargo publish
