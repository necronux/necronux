// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use thiserror::Error;

pub type Result<T> = std::result::Result<T, LogError>;

#[derive(Debug, Error)]
pub enum LogError {
    #[error(transparent)]
    SetGlobalSubscriberError(#[from] tracing::subscriber::SetGlobalDefaultError),

    #[error(transparent)]
    LogTracerInitError(#[from] tracing_log::log::SetLoggerError),
}
