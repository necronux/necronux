// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::IoError;
use necronux_macros::trace_instrument;
use std::{
    io::{Read, Write},
    result as stdrt,
};
use tracing::debug;

#[trace_instrument(level = "debug", skip(reader, writer), fields(reader_label = %reader_label, writer_label = %writer_label))]
pub fn copy_stream<R: Read, W: Write>(
    reader: &mut R,
    reader_label: &str,
    writer: &mut W,
    writer_label: &str,
) -> stdrt::Result<u64, IoError> {
    debug!(
        reader_label = %reader_label,
        writer_label = %writer_label,
        "Attempting to copy stream..."
    );

    let bytes_copied = std::io::copy(reader, writer).map_err(|e| IoError::CopyStreamError {
        reader_label: reader_label.to_string(),
        writer_label: writer_label.to_string(),
        source: e,
    })?;

    debug!(
        reader_label = %reader_label,
        writer_label = %writer_label,
        bytes_copied = %bytes_copied,
        "Successfully copied stream"
    );
    Ok(bytes_copied)
}
