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
    reuse spdx -o reuse.spdx

runpkl:
    just resolve-internalci
    just resolve-stdschema
    just eval-internalci
    just eval-stdschema
    just test-stdschema

# Following subcommands are used in ci

# internal tools

resolve-internalci:
    ./gradlew resolveInternalCI

eval-internalci:
    ./gradlew evalWorkflowsHelpers
    ./gradlew evalWorkflowsModules
    ./gradlew evalWorkflows

# stdschema

resolve-stdschema:
    ./gradlew resolveStdSchema

eval-stdschema:
    ./gradlew evalStdSchema

test-stdschema:
    ./gradlew testStdSchema

make-stdschemapkg:
    ./gradlew makeStdSchemaPkg
