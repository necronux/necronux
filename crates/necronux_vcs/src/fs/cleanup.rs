// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

pub fn delete_active_clone() -> anyhow::Result<()> {
    let path = crate::paths::active_clone_path()?;
    necronux_utils::fs::remove_dir_all(&path, "active clone directory")
}

pub fn delete_backup_active_clone() -> anyhow::Result<()> {
    let path = crate::paths::backup_active_clone_path()?;
    necronux_utils::fs::remove_dir_all(&path, "active clone backup directory")
}
