# ==-----------------------------------------------------------== #
# SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
#
# SPDX-License-Identifier: GPL-3.0-or-later
# ==-----------------------------------------------------------== #

rundebug:
    cargo clippy --workspace --all-targets --all-features --no-deps -- -D clippy::all -D clippy::cargo -D warnings
    cargo run
    reuse lint

runrelease:
    cargo clippy --workspace --all-targets --all-features --no-deps -- -D clippy::all -D clippy::cargo -D warnings
    cargo run --release
    reuse lint

lint:
    cargo clippy --workspace --all-targets --all-features --no-deps -- -D clippy::all -D clippy::cargo -D warnings
    reuse lint

evalinternaltool:
    ./gradlew evalWorkflowsGPL
    ./gradlew evalWorkflowsApache

sbom:
    reuse lint
    reuse spdx -o docs/reuse.spdx
