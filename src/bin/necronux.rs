// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

fn main() -> color_eyre::eyre::Result<()> {
    necronux_internal::cli_app::init()?;
    Ok(())
}
