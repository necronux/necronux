// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::AppContext;
#[cfg(all(feature = "necronux_log", debug_assertions))]
use crate::CliLogOutputFormat;

pub struct LogContext<'l> {
    pub app_ctx: &'l AppContext,
}

impl<'l> LogContext<'l> {
    pub fn new(app_ctx: &'l AppContext) -> Self {
        Self { app_ctx }
    }
}

#[cfg(all(feature = "necronux_log", debug_assertions))]
impl From<CliLogOutputFormat> for Option<necronux::log::StdErrLayer> {
    fn from(format: CliLogOutputFormat) -> Self {
        match format {
            CliLogOutputFormat::Fmt => Some(necronux::log::StdErrLayer::Fmt),
            CliLogOutputFormat::FmtExtended => Some(necronux::log::StdErrLayer::FmtExtended),
            CliLogOutputFormat::FancyTree => Some(necronux::log::StdErrLayer::FancyTree),
        }
    }
}
