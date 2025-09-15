// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use anyhow::Result;
use std::io::Write;

pub fn write_msg_to<W: Write>(w: &mut W, content: &str) -> Result<()> {
    writeln!(w, "{content}")?;
    Ok(())
}
