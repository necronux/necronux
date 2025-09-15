// ==----------------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2024-2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==----------------------------------------------------------------== //

#[macro_use]
mod messages;

#[path = "06_handle.rs"]
mod handle;
#[cfg(feature = "necronux_log")]
#[path = "04_log.rs"]
mod log;
#[path = "01_parse.rs"]
mod parse;
#[path = "03_run.rs"]
mod run;
#[path = "00_start.rs"]
mod start;
#[path = "02_sysinfo.rs"]
mod sysinfo;
#[path = "05_ui.rs"]
mod ui;

mod app;
mod app_config;
mod app_context;
mod cli_options;
mod handlers;
#[cfg(feature = "necronux_log")]
mod log_context;
mod subcommands;
pub mod theme;
mod ui_context;
mod utils;

pub use app::*;
pub use app_config::*;
pub use app_context::*;
pub use cli_options::*;
pub use handlers::*;
#[cfg(feature = "necronux_log")]
pub use log_context::*;
pub use start::*;
pub use subcommands::*;
pub use ui_context::*;
