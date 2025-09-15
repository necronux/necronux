// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::{kind::MsgKind, msg_body::MsgFragment};
use crate::{CliOutputFormat, CliProgressLevel, theme::ThemedUi, utils};
use anyhow::Result;
use indicatif::ProgressBar;
use tracing::info;

pub struct MsgParams<'prm> {
    pub fragments: &'prm [MsgFragment],
    pub json_fields: Option<serde_json::Map<String, serde_json::Value>>,
    pub emit_as_json: bool,
}

impl<'prm> MsgParams<'prm> {
    pub fn new(
        fragments: &'prm [MsgFragment],
        json_fields: Option<serde_json::Map<String, serde_json::Value>>,
        emit_as_json: bool,
    ) -> Self {
        MsgParams {
            fragments,
            json_fields,
            emit_as_json,
        }
    }
}

pub trait IsStderr: std::io::Write {}
impl IsStderr for std::io::Stderr {}
impl<'a> IsStderr for std::io::StderrLock<'a> {}

pub trait IsStdout: std::io::Write {}
impl IsStdout for std::io::Stdout {}
impl<'a> IsStdout for std::io::StdoutLock<'a> {}

impl<'t> ThemedUi<'t> {
    pub fn new_stderr_msg(
        &self,
        w: &mut impl IsStderr,
        kind: MsgKind,
        fragments: &[MsgFragment],
    ) -> Result<Option<ProgressBar>> {
        self.new_msg(w, kind, MsgParams::new(fragments, None, false))
    }

    pub fn new_stdout_msg(
        &self,
        w: &mut impl IsStdout,
        kind: MsgKind,
        fragments: &[MsgFragment],
        json_fields: Option<serde_json::Map<String, serde_json::Value>>,
    ) -> Result<Option<ProgressBar>> {
        self.new_msg(w, kind, MsgParams::new(fragments, json_fields, true))
    }

    fn new_msg(
        &self,
        w: &mut impl std::io::Write,
        kind: MsgKind,
        params: MsgParams<'_>,
    ) -> Result<Option<ProgressBar>> {
        match self.0.app_ctx.config.progress_level {
            CliProgressLevel::Detailed => {}
            CliProgressLevel::Minimal => {
                // Suppress step messages when minimal level
                if matches!(kind, MsgKind::Step { .. }) {
                    return Ok(None);
                }
            }
            CliProgressLevel::None => {
                // Suppress everything except stdout messages
                if !kind.is_stdout_msg_kind() {
                    return Ok(None);
                }
            }
        }

        let prefix = kind.prefix(self);
        let spinner_fn = kind.spinner_fn(self);

        self.write_msg(w, kind, prefix, params, spinner_fn)
    }

    fn write_msg<F>(
        &self,
        w: &mut impl std::io::Write,
        kind: MsgKind,
        prefix: String,
        params: MsgParams<'_>,
        spinner_fn: Option<F>,
    ) -> Result<Option<ProgressBar>>
    where
        F: Fn(&ThemedUi<'t>, String) -> Result<ProgressBar>,
    {
        let body = self.new_msg_body(params.fragments);
        let full_msg = format!("{} {}", prefix, &body);
        let timestamp = chrono::Utc::now().to_rfc3339();

        info!(
            timestamp = %timestamp,
            kind = %kind.as_str(),
            prefix = %prefix,
            body = %body,
            extra = ?params.json_fields,
            "{full_msg}",
        );

        match self.0.app_ctx.config.output_format {
            CliOutputFormat::Human => {
                if let Some(spinner_fn) = spinner_fn {
                    let pb = spinner_fn(self, body)?;
                    Ok(Some(pb))
                } else {
                    utils::write_msg_to(w, &full_msg)?;
                    Ok(None)
                }
            }
            CliOutputFormat::Json => {
                if params.emit_as_json {
                    let mut msg_obj = serde_json::Map::new();

                    msg_obj.insert("timestamp".into(), serde_json::Value::String(timestamp));
                    msg_obj.insert(
                        "kind".into(),
                        serde_json::Value::String(kind.as_str().into()),
                    );
                    msg_obj.insert("prefix".into(), serde_json::Value::String(prefix));
                    msg_obj.insert("body".into(), serde_json::Value::String(body));

                    if let Some(extra) = params.json_fields {
                        msg_obj.extend(extra);
                    }

                    let mut root_map = serde_json::Map::new();
                    root_map.insert("message".into(), serde_json::Value::Object(msg_obj));

                    let root = serde_json::Value::Object(root_map);

                    serde_json::to_writer(&mut *w, &root)?;
                    new_line!(w)?;
                    Ok(None)
                } else {
                    utils::write_msg_to(w, &full_msg)?;
                    Ok(None)
                }
            }
        }
    }
}
