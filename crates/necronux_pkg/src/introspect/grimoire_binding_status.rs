// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::{PkgError, Result};
use std::path::PathBuf;
use tracing::{debug, info, warn};

#[derive(Debug)]
pub struct GrimoireBindingStatus {
    pub is_bound: bool,
    pub package_name: Option<String>,
    pub package_version: Option<String>,
    pub grimoire_path: Option<PathBuf>,
}

impl GrimoireBindingStatus {
    pub fn introspect() -> Result<Self> {
        #[cfg(feature = "trace")]
        let _span = tracing::debug_span!("introspect_grimoire_binding_status").entered();

        info!("Introspecting grimoire binding status...",);

        fn introspect_inner() -> Result<GrimoireBindingStatus> {
            let mut status = GrimoireBindingStatus {
                is_bound: false,
                package_name: None,
                package_version: None,
                grimoire_path: None,
            };

            let dir = match necronux_utils::paths::current_grimoire_path() {
                Ok(d) => d,
                Err(e) => {
                    info!("{e}");
                    return Ok(status);
                }
            };

            let entries = match necronux_utils::fs::read_dir_if_exists(&dir, "current grimoire") {
                Ok(e) => e,
                Err(e) => {
                    info!("{e}");
                    return Ok(status);
                }
            };

            let zip_paths: Vec<PathBuf> = entries
                .into_iter()
                .filter_map(|r| r.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|ext| ext == "zip"))
                .collect();

            match zip_paths.len() {
                0 => {
                    debug!("No .zip file found in current grimoire directory");
                    return Ok(status);
                }
                1 => {}
                _ => {
                    warn!(
                        "Multiple .zip files found in current grimoire directory: {:?}",
                        zip_paths
                    );
                    return Ok(status);
                }
            }

            let zip_path = &zip_paths[0];

            let Some(file_name) = zip_path.file_name().and_then(|f| f.to_str()) else {
                warn!("Zip filename is not valid UTF-8: '{zip_path:?}'");
                return Ok(status);
            };

            let Some(stem) = file_name.strip_suffix(".zip") else {
                warn!("Zip file does not have a '.zip' extension: {file_name}");
                return Ok(status);
            };

            let Some((name, ver)) = stem.split_once('@') else {
                warn!("Zip filename is not in '<name>@<version>.zip' format: {file_name}");
                return Ok(status);
            };

            let extracted_dir = dir.join(name);
            let grimoire_file = extracted_dir.join("necronux.grimoire");

            if grimoire_file.exists() {
                status.is_bound = true;
                status.package_name = Some(name.to_string());
                status.package_version = Some(ver.to_string());
                status.grimoire_path = Some(grimoire_file.to_path_buf());
            }

            debug!("Successfully introspected grimoire binding status",);
            Ok(status)
        }

        introspect_inner().map_err(|e| PkgError::IntrospectGrimoireBindingStatusError {
            source: Box::new(e),
        })
    }
}
