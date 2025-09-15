// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::layers::CHROME_GUARD;
use necronux_utils::trace_instrument;
use tracing::debug;

#[trace_instrument(level = "debug")]
pub fn maybe_flush() {
    debug!("Flushing tracing span guard...");

    if let Some(guard_lock) = CHROME_GUARD.get() {
        if let Ok(mut guard_opt) = guard_lock.lock() {
            if let Some(guard) = guard_opt.take() {
                guard.flush(); // Ensures full flush
            }
        }
    }

    debug!("Flushed tracing span guard successfully");
}
