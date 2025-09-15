// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::theme::ThemedUi;
use console::StyledObject;
use std::borrow::Cow;

pub struct MsgFragment {
    pub text: Cow<'static, str>,
    pub style_fn: fn(&str) -> StyledObject<&str>,
}

impl MsgFragment {
    pub fn new<T: Into<Cow<'static, str>>>(
        text: T,
        style_fn: fn(&str) -> StyledObject<&str>,
    ) -> Self {
        Self {
            text: text.into(),
            style_fn,
        }
    }
}

impl<'t> ThemedUi<'t> {
    pub fn new_msg_body(&self, fragments: &[MsgFragment]) -> String {
        self.format_msg_body(fragments)
    }

    fn format_msg_body(&self, fragments: &[MsgFragment]) -> String {
        fragments
            .iter()
            .map(|f| format!("{}", (f.style_fn)(&f.text)))
            .collect::<String>()
    }
}
