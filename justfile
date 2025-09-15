# ==-----------------------------------------------------------== #
# SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
#
# SPDX-License-Identifier: Apache-2.0
# ==-----------------------------------------------------------== #

run-debug:
    cargo clippy --workspace --all-targets --all-features --no-deps -- -D clippy::all -D clippy::cargo -D warnings
    cargo run
    reuse lint

run-release:
    cargo clippy --workspace --all-targets --all-features --no-deps -- -D clippy::all -D clippy::cargo -D warnings
    cargo run --release
    reuse lint

lint:
    cargo clippy --workspace --all-targets --all-features --no-deps -- -D clippy::all -D clippy::cargo -D warnings
    reuse lint

sbom:
    reuse lint
    reuse spdx -o reuse.spdx

runpkl:
    just resolve-clipklci
    just eval-clipklci

# Following subcommands are used in ci

resolve-clipklci:
    ./gradlew resolveCliPklCi

eval-clipklci:
    ./gradlew evalCliPklCiModules
    ./gradlew evalCliPklCiWorkflows
