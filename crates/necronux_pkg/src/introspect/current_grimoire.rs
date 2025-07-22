// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::{PkgError, Result};
use std::path::PathBuf;
use tracing::{debug, info, warn};

#[derive(Debug)]
pub struct CurrentGrimoireInfo {
    pub package_name: String,
    pub package_version: String,
    pub fetched_package_zip_path: PathBuf,
}

impl CurrentGrimoireInfo {
    pub fn introspect() -> Result<Self> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("introspect_current_grimoire").entered();

        info!("Introspecting current grimoire...",);

        fn introspect_inner() -> Result<CurrentGrimoireInfo> {
            let dir = necronux_utils::paths::current_grimoire_path()?;
            let entries = necronux_utils::fs::read_dir_if_exists(&dir, "current grimoire")?;
            let mut zips = Vec::new();

            for entry in entries {
                match entry {
                    Ok(en) => {
                        let path = en.path();
                        if let Some(ext) = path.extension() {
                            if ext == "zip" {
                                zips.push(en);
                            }
                        }
                    }
                    Err(err) => {
                        warn!("Skipping directory entry due to error: {err}");
                    }
                }
            }

            let zip_entry = match zips.len() {
                0 => return Err(PkgError::NoZipFound),
                1 => zips.into_iter().next().ok_or(PkgError::NoZipFound)?,
                _ => return Err(PkgError::MultipleZips),
            };

            let zip_path = zip_entry.path();

            let file_name = zip_path
                .file_name()
                .and_then(|f| f.to_str())
                .ok_or_else(|| PkgError::NonUtf8FileName {
                    path: zip_path.to_path_buf(),
                })?;

            let stem =
                file_name
                    .strip_suffix(".zip")
                    .ok_or_else(|| PkgError::MissingZipExtension {
                        file_name: file_name.to_string(),
                    })?;

            let (package_name, version) =
                stem.split_once('@')
                    .ok_or_else(|| PkgError::InvalidZipFileNameFormat {
                        file_name: file_name.to_string(),
                    })?;

            if package_name.is_empty() || version.is_empty() {
                Err(PkgError::InvalidZipFileNameFormat {
                    file_name: file_name.to_string(),
                })?
            }

            let info = CurrentGrimoireInfo {
                package_name: package_name.to_string(),
                package_version: version.to_string(),
                fetched_package_zip_path: zip_path.to_path_buf(),
            };

            debug!(
                "Successfully introspected current grimoire at '{}'",
                zip_path.display(),
            );
            Ok(info)
        }

        introspect_inner().map_err(|e| PkgError::IntrospectCurrentGrimoireError {
            source: Box::new(e),
        })
    }
}
