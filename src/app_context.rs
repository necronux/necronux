// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::{AppConfig, CliOptions, CliOutputFormat, ColorChoice, sysinfo::SystemInfo};

pub struct AppContext {
    pub cli_opts: CliOptions,
    pub config: AppConfig,
    pub sys: SystemInfo,
}

impl AppContext {
    pub fn new(cli_opts: CliOptions, config: AppConfig, sys: SystemInfo) -> Self {
        Self {
            cli_opts,
            config,
            sys,
        }
    }
}

impl AppContext {
    pub fn should_use_color_stdout(&self) -> bool {
        match self.config.color {
            ColorChoice::Always => true,
            ColorChoice::Never => false,
            ColorChoice::Auto => match self.config.output_format {
                CliOutputFormat::Human => self.sys.stdout_is_tty && !self.is_term_dumb(),
                _ => false,
            },
        }
    }
    pub fn should_use_color_stderr(&self) -> bool {
        match self.config.color {
            ColorChoice::Always => true,
            ColorChoice::Never => false,
            ColorChoice::Auto => match self.config.output_format {
                CliOutputFormat::Human => self.sys.stderr_is_tty && !self.is_term_dumb(),
                _ => false,
            },
        }
    }

    pub fn should_use_unicode_stdout(&self) -> bool {
        self.config.unicode
    }
    pub fn should_use_unicode_stderr(&self) -> bool {
        self.config.unicode
    }

    pub fn is_term_dumb(&self) -> bool {
        matches!(self.sys.term.as_deref(), Some("dumb"))
    }
}
