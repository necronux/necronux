# ==-----------------------------------------------------------== #
# SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
#
# SPDX-License-Identifier: GPL-3.0-or-later
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
    reuse spdx -o docs/reuse.spdx

runpkl:
    # internal tools
    just resolve-internalpkldeps
    just eval-internalpkl
    # stdlib
    just resolve-stdlibpkldeps
    just eval-stdlibpkl
    just test-stdlibpkl

# Following subcommands are used in ci

# internal tools
resolve-internalpkldeps:
    ./gradlew resolveInternalPklApacheDeps
    ./gradlew resolveInternalPklGPLDeps

eval-internalpkl:
    ./gradlew evalWorkflowsCommonsGPL
    ./gradlew evalWorkflowsGPL
    ./gradlew evalWorkflowsApache

# stdlib
resolve-stdlibpkldeps:
    ./gradlew resolveStdlibPklDeps

eval-stdlibpkl:
    ./gradlew evalStdlibPkl

test-stdlibpkl:
    ./gradlew testStdlibPkl

make-stdlibpklpkg:
    ./gradlew makeStdlibPklPkg
