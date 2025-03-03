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
    just resolve-internalci
    just resolve-stdschemapkl
    just eval-internalci
    just eval-stdschemapkl
    just test-stdschemapkl

# Following subcommands are used in ci

# internal tools

resolve-internalci:
    ./gradlew resolveInternalCIApache
    ./gradlew resolveInternalCIGPL

eval-internalci:
    ./gradlew evalWorkflowsCommonsGPL
    ./gradlew evalWorkflowsGPL
    ./gradlew evalWorkflowsApache

# stdschema

resolve-stdschemapkl:
    ./gradlew resolveStdSchemaPkl

eval-stdschemapkl:
    ./gradlew evalStdSchemaPkl

test-stdschemapkl:
    ./gradlew testStdSchemaPkl

make-stdschemapklpkg:
    ./gradlew makeStdSchemaPklPkg
