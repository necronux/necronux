use crate::bundle::schema::Bundle;
use color_eyre::eyre::{eyre, Context, Result};
use log::debug;
use std::{env, path::Path};

pub fn load_bundle_file() -> Result<Option<Bundle>> {
    debug!("Loading the necronux bundle file...");

    let config_dir = env::var("XDG_CONFIG_HOME")
        .or_else(|_| {
            env::var("HOME").map(|home| {
                Path::new(&home)
                    .join(".config")
                    .to_string_lossy()
                    .to_string()
            })
        })
        .context("Neither $XDG_CONFIG_HOME nor $HOME are set")?;
    let bundle_file_path = Path::new(&config_dir).join("necronux/necronux.pkl");
    let bundle_file_path_str = bundle_file_path.to_str().ok_or_else(|| {
        eyre!(
            "Failed to convert the bundle file path to a valid UTF-8 string: {}",
            bundle_file_path.display()
        )
    })?;

    if !bundle_file_path.exists() {
        debug!(
            "Necronux bundle file not found at: {}",
            bundle_file_path_str
        );
        return Ok(None);
    }

    debug!("Found the necronux bundle file at {}", bundle_file_path_str);
    let bundle: Bundle = rpkl::from_config(bundle_file_path_str)
        .with_context(|| eyre!("Failed to parse file as PKL: {}", bundle_file_path_str))?;
    debug!("Necronux bundle file successfully loaded.");

    Ok(Some(bundle))
}
