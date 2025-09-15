// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use std::io::{IsTerminal, stderr, stdout};
use sysinfo::System;

pub struct SystemInfo {
    pub os_name: Option<String>,
    pub os_version: Option<String>,
    pub kernel_version: Option<String>,
    pub hostname: Option<String>,
    pub arch: String,
    pub cpu: String,
    pub num_cpu: usize,
    pub memory_total: u64,
    pub memory_used: u64,
    pub stdout_is_tty: bool,
    pub stderr_is_tty: bool,
    pub term: Option<String>,
    pub current_locale: String,
}

impl SystemInfo {
    pub fn collect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_cpu_all();
        sys.refresh_memory();

        let os_name = System::name();
        let os_version = System::os_version();
        let kernel_version = System::kernel_version();
        let hostname = System::host_name();

        let arch = std::env::consts::ARCH.to_string();

        let cpu = sys
            .cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "unknown".into());

        let num_cpu = sys.cpus().len();

        let memory_total = sys.total_memory();
        let memory_used = sys.used_memory();

        let stdout_is_tty = stdout().is_terminal();
        let stderr_is_tty = stderr().is_terminal();

        let term = std::env::var("TERM").ok();

        let current_locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));

        Self {
            os_name,
            os_version,
            kernel_version,
            hostname,
            arch,
            cpu,
            num_cpu,
            memory_total,
            memory_used,
            stdout_is_tty,
            stderr_is_tty,
            term,
            current_locale,
        }
    }
}
