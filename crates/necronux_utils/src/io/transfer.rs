// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::Result;
use crate::error::IoError;
use std::io::{Read, Write};
use tracing::debug;

pub fn copy_stream<R: Read, W: Write>(
    reader: &mut R,
    reader_label: &str,
    writer: &mut W,
    writer_label: &str,
) -> Result<u64> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!(
        "copy_stream",
        reader_label = reader_label,
        writer_label = writer_label
    )
    .entered();
    let bytes_copied = std::io::copy(reader, writer).map_err(|e| IoError::CopyStreamError {
        reader_label: reader_label.to_string(),
        writer_label: writer_label.to_string(),
        source: e,
    })?;
    debug!("Successfully copied from {reader_label} to {writer_label}");
    Ok(bytes_copied)
}
