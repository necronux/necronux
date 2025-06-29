// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

pub fn rename_active_clone_to_backup_active_clone() -> anyhow::Result<()> {
    let old_path = crate::paths::active_clone_path()?;
    let new_path = crate::paths::backup_active_clone_path()?;
    necronux_utils::fs::rename_dir(&old_path, "active clone directory", &new_path)
}

pub fn rename_backup_active_clone_to_active_clone() -> anyhow::Result<()> {
    let old_path = crate::paths::backup_active_clone_path()?;
    let new_path = crate::paths::active_clone_path()?;
    necronux_utils::fs::rename_dir(&old_path, "active clone backup directory", &new_path)
}
