// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

use crate::error::{ExtractZipError, ExtractZipErrorWithContext};
use necronux_macros::trace_instrument;
use std::{path::Path, result as stdrt};
use tracing::{debug, warn};

#[trace_instrument(level = "debug", fields(zip_path_label = %zip_path_label, zip_path = %zip_path.display(), dest_dir = %dest_dir.display()))]
pub fn extract_zip(
    zip_path: &Path,
    zip_path_label: &str,
    dest_dir: &Path,
) -> stdrt::Result<(), ExtractZipErrorWithContext> {
    debug!(
        zip_path_label = %zip_path_label,
        zip_path = %zip_path.display(),
        dest_dir = %dest_dir.display(),
        "Attempting to extract zip file...",
    );

    extract_zip_inner(zip_path, zip_path_label, dest_dir).map_err(|e| {
        ExtractZipErrorWithContext {
            zip_path_label: zip_path_label.to_string(),
            zip_path: zip_path.to_path_buf(),
            dest_path: dest_dir.to_path_buf(),
            source: Box::new(e),
        }
    })?;

    debug!(
        zip_path_label = %zip_path_label,
        zip_path = %zip_path.display(),
        dest_dir = %dest_dir.display(),
        "Successfully extracted zip file",
    );
    Ok(())
}

fn extract_zip_inner(
    zip_path: &Path,
    zip_path_label: &str,
    dest_dir: &Path,
) -> stdrt::Result<(), ExtractZipError> {
    let file = crate::fs::open_file(zip_path, zip_path_label)?;

    let mut archive = zip::ZipArchive::new(file).map_err(|e| ExtractZipError::ZipError {
        path: zip_path.to_path_buf(),
        label: zip_path_label.to_string(),
        source: e,
    })?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| ExtractZipError::ZipError {
            path: zip_path.to_path_buf(),
            label: zip_path_label.to_string(),
            source: e,
        })?;

        let outpath = match file.enclosed_name() {
            Some(path) => dest_dir.join(path),
            None => {
                warn!(
                        file_name = %file.name(),
                        "Skipping unsafe archive entry");
                continue;
            }
        };

        if file.is_dir() {
            crate::fs::create_dir_all(&outpath, "extraction output")?;
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
                file_name = %i,
                file_size_bytes = %file.size(),
                extracted_path = %outpath.display(),
                "File successfully extracted",
            );
        }

        #[cfg(unix)]
        {
            if let Some(mode) = file.unix_mode() {
                crate::fs::set_permissions(&outpath, "extraction output file", mode)?;
            }
        }
    }
    Ok(())
}
