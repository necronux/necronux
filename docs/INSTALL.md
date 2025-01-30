<!--
# ==-----------------------------------------------------------== #
# SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
#
# SPDX-License-Identifier: CC-BY-SA-4.0
# ==-----------------------------------------------------------== #
-->

# Installing from Source

**Note: This document describes _building_ Necronux _from source_.
This is _not recommended_ if you don't know what you're doing.
If you just want to install Necronux, check out the [README.md](README.md) instead.**

## Building the Source
### Dependencies
- Ensure you have [Rust](https://www.rust-lang.org/learn/get-started) installed. You can install Rust using `rustup`:
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation
- Clone the repository and navigate to the project directory:
```shell
git clone https://github.com/NayanTheSpaceGuy/necronux.git
cd necronux
```

### Building
- Compile the project with:
```shell
cargo build --release
```
- The compiled binary will be located in `target/release/necronux`.

## Running
```shell
./target/release/necronux [OPTIONS] [SUBCOMMAND]
```
To view the help message, you can use:
```shell
./target/release/necronux --help
```
Or simply:
```shell
./target/release/necronux
```
