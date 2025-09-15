// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{App, AppConfig, AppContext, CliOptions, sysinfo::SystemInfo};

pub struct Cli;

impl Cli {
    pub fn start() -> anyhow::Result<()> {
        let t0 = std::time::Instant::now();

        let cli_opts = CliOptions::parse();
        let sysinfo = SystemInfo::collect();
        let config = AppConfig::from(&cli_opts);
        let ctx = AppContext::new(cli_opts, config, sysinfo);
        let app = App::new(ctx);
        app.run(t0)
    }
}
