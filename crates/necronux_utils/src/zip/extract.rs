// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use super::Result;
use crate::error::ZipError;
use std::path::Path;
use tracing::{debug, warn};
use zip::ZipArchive;

pub fn extract_zip_if_exists(zip_path: &Path, zip_path_label: &str, dest_dir: &Path) -> Result<()> {
    #[cfg(feature = "trace")]
    let _span = tracing::debug_span!("extract_zip", zip_path_label = zip_path_label).entered();

    fn extract_zip_inner(zip_path: &Path, zip_path_label: &str, dest_dir: &Path) -> Result<()> {
        let file = crate::fs::open_file_if_exists(zip_path, zip_path_label)?;

        let mut archive = ZipArchive::new(file).map_err(|e| ZipError::ReadFileError {
            path: zip_path.to_path_buf(),
            label: zip_path_label.to_string(),
            source: e,
        })?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| ZipError::ReadFileError {
                path: zip_path.to_path_buf(),
                label: zip_path_label.to_string(),
                source: e,
            })?;

            let outpath = match file.enclosed_name() {
                Some(path) => dest_dir.join(path),
                None => {
                    warn!("Skipping unsafe archive entry: {}", file.name());
                    continue;
                }
            };

            if file.is_dir() {
                crate::fs::create_dir_all(&outpath, "extraction output")?;
                debug!("File {i} extracted to '{}'", outpath.display());
            } else {
                if let Some(parent) = outpath.parent() {
                    crate::fs::create_dir_all(parent, "extraction output parent")?;
                }

                let mut outfile = crate::fs::create_file(&outpath, "extraction output")?;

                crate::io::copy_stream(
                    &mut file,
                    "archive entry file",
                    &mut outfile,
                    "extraction output file",
                )?;

                debug!(
                    "File {i} extracted to '{}' ({} bytes)",
                    outpath.display(),
                    file.size()
                );
            }

            #[cfg(unix)]
            {
                if let Some(mode) = file.unix_mode() {
                    crate::fs::set_permissions_if_exists(&outpath, "extraction output file", mode)?;
                }
            }
        }
        debug!(
            "Successfully extracted {zip_path_label} at '{}' to '{}'",
            zip_path.display(),
            dest_dir.display()
        );
        Ok(())
    }

    extract_zip_inner(zip_path, zip_path_label, dest_dir).map_err(|e| ZipError::ExtractError {
        zip_path_label: zip_path_label.to_string(),
        zip_path: zip_path.to_path_buf(),
        dest_path: dest_dir.to_path_buf(),
        source: Box::new(e),
    })
}
