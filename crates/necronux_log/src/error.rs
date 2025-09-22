// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use thiserror::Error;

#[derive(Debug, Error)]
#[error("Failed to initialize logging")]
pub struct InitializeLoggingErrorWithContext {
    #[source]
    pub source: InitializeLoggingError,
}
#[derive(Debug, Error)]
pub enum InitializeLoggingError {
    #[error(transparent)]
    SetGlobalSubscriberError(#[from] tracing::subscriber::SetGlobalDefaultError),
    #[error(transparent)]
    InitializeLogTracerError(#[from] tracing_log::log::SetLoggerError),
    #[error(transparent)]
    SetLayerError(#[from] SetLayerError),
}

#[derive(Debug, Error)]
pub enum SetLayerError {
    #[error("Failed to set chrome layer guard, already set")]
    ChromeLayerGuardAlreadySet,
    #[error(transparent)]
    PathError(#[from] necronux_utils::error::PathError),
    #[error(transparent)]
    FsError(#[from] necronux_utils::error::FsError),
}
