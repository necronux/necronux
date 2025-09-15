// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{LogSetup, error::SetLayerError};
use once_cell::sync::OnceCell;
use std::result as stdrt;
use std::sync::Mutex;
use tracing::Subscriber;
use tracing_chrome::{ChromeLayerBuilder, FlushGuard};
use tracing_subscriber::{Layer, registry::LookupSpan};

pub static CHROME_GUARD: OnceCell<Mutex<Option<FlushGuard>>> = OnceCell::new();

impl LogSetup {
    pub fn chrome_layer_with_guard<S>(&self) -> stdrt::Result<impl Layer<S>, SetLayerError>
    where
        S: Subscriber + for<'span> LookupSpan<'span> + Send + Sync,
    {
        let (chrome_layer, guard) = ChromeLayerBuilder::new()
            .file("target/trace.json")
            .include_args(true)
            .build();

        CHROME_GUARD
            .set(Mutex::new(Some(guard)))
            .map_err(|_| SetLayerError::SetChromeLayerGuardError)?;

        Ok(chrome_layer)
    }
}
