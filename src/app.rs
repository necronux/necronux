// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::AppContext;

pub struct App {
    pub ctx: AppContext,
}

impl App {
    pub fn new(ctx: AppContext) -> Self {
        Self { ctx }
    }
}
