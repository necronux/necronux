// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::{IntrospectCurrentGrimoireError, IntrospectCurrentGrimoireErrorWithContext};
use necronux_utils::trace_instrument;
use std::{path::PathBuf, result as stdrt};
use tracing::{debug, warn};

#[derive(Debug)]
pub struct CurrentGrimoireInfo {
    pub package_name: String,
    pub package_version: String,
    pub fetched_package_zip_path: PathBuf,
}

impl CurrentGrimoireInfo {
    #[trace_instrument(level = "debug", name = "CurrentGrimoireInfo::introspect")]
    pub fn introspect() -> stdrt::Result<Self, IntrospectCurrentGrimoireErrorWithContext> {
        debug!("Introspecting current grimoire...");

        let info = introspect_inner()
            .map_err(|e| IntrospectCurrentGrimoireErrorWithContext { source: e })?;

        debug!(
            fetched_package_zip_path = %info.fetched_package_zip_path.display(),
            "Successfully introspected current grimoire",
        );
        Ok(info)
    }
}

fn introspect_inner() -> stdrt::Result<CurrentGrimoireInfo, IntrospectCurrentGrimoireError> {
    let dir = necronux_utils::paths::current_grimoire_path()?;
    let entries = necronux_utils::fs::read_dir(&dir, "current grimoire")?;

    let mut zips = Vec::new();
    for entry in entries {
        match entry {
            Ok(en) => {
                let path = en.path();
                if let Some(ext) = path.extension() {
                    if ext == "zip" {
                        zips.push(en);
                        if zips.len() > 1 {
                            return Err(IntrospectCurrentGrimoireError::MultipleZipFiles {
                                path: dir,
                            });
                        }
                    }
                }
            }
            Err(err) => {
                warn!(error = %err, "Skipping directory entry due to error");
            }
        }
    }

    let zip_entry = match zips.len() {
        0 => return Err(IntrospectCurrentGrimoireError::NoZipFileFound { path: dir }),
        1 => zips
            .into_iter()
            .next()
            .ok_or(IntrospectCurrentGrimoireError::NoZipFileFound { path: dir })?,
        _ => return Err(IntrospectCurrentGrimoireError::MultipleZipFiles { path: dir }),
    };

    let zip_path = zip_entry.path();

    let file_name = zip_path
        .file_name()
        .and_then(|f| f.to_str())
        .ok_or_else(|| IntrospectCurrentGrimoireError::NonUtf8FileName {
            path: zip_path.to_path_buf(),
        })?;

    let stem = file_name.strip_suffix(".zip").ok_or_else(|| {
        IntrospectCurrentGrimoireError::MissingZipExtension {
            file_name: file_name.to_string(),
        }
    })?;

    let (package_name, version) = stem.split_once('@').ok_or_else(|| {
        IntrospectCurrentGrimoireError::InvalidZipFileNameFormat {
            file_name: file_name.to_string(),
        }
    })?;

    if package_name.is_empty() || version.is_empty() {
        return Err(IntrospectCurrentGrimoireError::InvalidZipFileNameFormat {
            file_name: file_name.to_string(),
        });
    }

    let info = CurrentGrimoireInfo {
        package_name: package_name.to_string(),
        package_version: version.to_string(),
        fetched_package_zip_path: zip_path.to_path_buf(),
    };
    Ok(info)
}
