// ==----------------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2024-2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==----------------------------------------------------------------== //

mod commands;
mod flow_controller;
mod handlers;
mod logger;

use color_eyre::eyre::Result;

pub fn init() -> Result<()> {
    // Initializes the flow controller.
    flow_controller::init_flow_controller()?;

    Ok(())
}
