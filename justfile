# ==-----------------------------------------------------------== #
# SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
#
# SPDX-License-Identifier: GPL-3.0-or-later
# ==-----------------------------------------------------------== #

run_all:
    cargo clippy
    cargo run
    cargo test
    reuse lint
    reuse spdx -o docs/reuse.spdx

run:
    cargo clippy
    cargo run
    reuse lint

build:
    cargo build

test:
    cargo test

lint:
    cargo clippy
    reuse lint
    reuse spdx -o docs/reuse.spdx
