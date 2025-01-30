// ==----------------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2024-2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==----------------------------------------------------------------== //

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

mod commands;
mod flow_controller;
mod handlers;
mod logger;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    // Initializes the flow controller.
    flow_controller::init_flow_controller()?;

    Ok(())
}
