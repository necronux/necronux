# ==-----------------------------------------------------------== #
# SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
#
# SPDX-License-Identifier: GPL-3.0-or-later
# ==-----------------------------------------------------------== #

nrun:
    cargo clippy
    cargo run
    reuse lint

nrunr:
    cargo clippy
    cargo run --release
    reuse lint

nlint:
    cargo clippy
    reuse lint

sbom:
    reuse lint
    reuse spdx -o docs/reuse.spdx
