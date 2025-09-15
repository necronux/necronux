// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::AppContext;

pub struct UiContext<'u> {
    pub app_ctx: &'u AppContext,
}

impl<'u> UiContext<'u> {
    pub fn new(app_ctx: &'u AppContext) -> Self {
        Self { app_ctx }
    }
}
