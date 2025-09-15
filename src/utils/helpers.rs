// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::sysinfo::SystemInfo;
use anyhow::Result;
use tracing::info;

pub fn remove_current_grimoire_dir() -> Result<()> {
    let current_grimoire_path = necronux::utils::paths::current_grimoire_path()?;
    necronux::utils::fs::remove_dir_all(&current_grimoire_path, "current grimoire")?;
    Ok(())
}

pub fn log_sys_info(sys: &SystemInfo) {
    info!(
        os = %sys.os_name.as_deref().unwrap_or("unknown"),
        os_version = %sys.os_version.as_deref().unwrap_or("unknown"),
        kernel_version = %sys.kernel_version.as_deref().unwrap_or("unknown"),
        hostname = %sys.hostname.as_deref().unwrap_or("unknown"),
        arch = %sys.arch,
        cpu = %sys.cpu,
        num_cpu = sys.num_cpu,
        memory_used_mib = sys.memory_used / 1024 / 1024,
        memory_total_mib = sys.memory_total / 1024 / 1024,
        stdout_is_tty = sys.stdout_is_tty,
        stderr_is_tty = sys.stderr_is_tty,
        term = %sys.term.as_deref().unwrap_or("unset"),
        current_locale = %sys.current_locale,
        "Retrieved system info",
    );
}
