// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use thiserror::Error;

// Crate level error

#[derive(Debug, Error)]
pub enum LogError {
    #[error("Failed to initialize logging")]
    InitializeError {
        #[source]
        source: InitializeError,
    },
}

// Function level errors

#[derive(Debug, Error)]
pub enum InitializeError {
    #[error(transparent)]
    SetGlobalSubscriberError(#[from] tracing::subscriber::SetGlobalDefaultError),

    #[error(transparent)]
    LogTracerInitError(#[from] tracing_log::log::SetLoggerError),

    #[error(transparent)]
    SetLayerError(#[from] SetLayerError),
}

#[derive(Debug, Error)]
pub enum SetLayerError {
    #[error("Failed to set chrome layer guard, already set")]
    SetChromeLayerGuardError,

    #[error(transparent)]
    PathError(#[from] necronux_utils::error::PathError),

    #[error(transparent)]
    FsError(#[from] necronux_utils::error::FsError),
}
