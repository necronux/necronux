#!/bin/bash

# ==-----------------------------------------------------------== #
# SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
#
# SPDX-License-Identifier: Apache-2.0
# ==-----------------------------------------------------------== #
#
# Check if reuse is installed
if ! reuse --version &> /dev/null
then
    echo "Error: reuse is not installed. Install it first."
    exit 1
fi

# Define the output file for the SBOM (optional)
OUTPUT_FILE="docs/reuse.spdx"

# Run reuse sbom and save to the output file
if reuse spdx --output "$OUTPUT_FILE"; then
    echo "SBOM generated successfully: $OUTPUT_FILE"
    exit 0
else
    echo "Error: Failed to generate SBOM"
    exit 1
fi
