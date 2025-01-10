#[cfg(unix)]
use color_eyre::eyre::eyre;
use color_eyre::eyre::Result;
#[cfg(unix)]
use log::debug;
#[cfg(unix)]
use std::{env, path::PathBuf, process::Command};
#[cfg(unix)]
use std::{fs::Permissions, os::unix::fs::PermissionsExt};

pub fn trinity_heliolens_part_one() -> Result<()> {
    #[cfg(unix)]
    {
        setup_environment("one")?
    }
    #[cfg(windows)]
    {
        println!("This setup is not supported yet for Windows.");
    }
    Ok(())
}

pub fn trinity_heliolens_part_two() -> Result<()> {
    #[cfg(unix)]
    {
        setup_environment("two")?
    }
    #[cfg(windows)]
    {
        println!("This setup is not supported yet for Windows.");
    }
    Ok(())
}

#[cfg(unix)]
fn setup_environment(part: &str) -> Result<()> {
    let config_dir: PathBuf =
        env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|_| -> Result<PathBuf> {
                let home_dir = env::var("HOME")
                    .map(PathBuf::from)
                    .map_err(|_| eyre!("HOME environment variable not set"))?;
                Ok(home_dir.join(".config"))
            })?;

    let project_dir = config_dir.join("necronux");
    let dot_dir = project_dir.join("dot");

    setup_directories(&project_dir, &dot_dir)?;
    clone_repository(&dot_dir)?;

    let script_path = dot_dir.join(format!(
        "homelab/scripts/trinity/heliolens_setup_part_{}.sh",
        part
    ));
    execute_script(&script_path)?;

    debug!("Script executed successfully");
    Ok(())
}

#[cfg(unix)]
fn setup_directories(project_dir: &PathBuf, dot_dir: &PathBuf) -> Result<()> {
    if !project_dir.exists() {
        std::fs::create_dir_all(project_dir)?;
        debug!("Created project directory at {:?}", project_dir);
    }

    match (
        dot_dir.exists(),
        dot_dir
            .read_dir()
            .map(|mut d| d.next().is_some())
            .unwrap_or(false),
    ) {
        (true, true) => {
            std::fs::remove_dir_all(dot_dir)?;
            debug!(
                "Removed existing non-empty 'dot' directory at {:?}",
                dot_dir
            );
            std::fs::create_dir_all(dot_dir)?;
            debug!("Created new 'dot' directory at {:?}", dot_dir);
        }
        (false, _) => {
            std::fs::create_dir_all(dot_dir)?;
            debug!("Created 'dot' directory at {:?}", dot_dir);
        }
        _ => {}
    }

    Ok(())
}

#[cfg(unix)]
fn clone_repository(dot_dir: &PathBuf) -> Result<()> {
    let repo_url = "https://github.com/NayanTheSpaceGuy/dotlab.git";
    let git_clone_status = Command::new("git")
        .arg("clone")
        .arg(repo_url)
        .arg(dot_dir)
        .status()
        .map_err(|e| eyre!("Failed to execute git command: {:?}", e))?;

    if !git_clone_status.success() {
        return Err(eyre!(
            "Git clone failed with status: {:?}",
            git_clone_status
        ));
    }
    debug!("Successfully cloned repository into {:?}", dot_dir);
    Ok(())
}

#[cfg(unix)]
fn execute_script(script_path: &PathBuf) -> Result<()> {
    debug!(
        "Attempting to change permissions of script: {:?}",
        script_path
    );
    std::fs::set_permissions(script_path, Permissions::from_mode(0o755)).map_err(|e| {
        eyre!(
            "Failed to set permissions for script {:?}: {:?}",
            script_path,
            e
        )
    })?;
    debug!("Successfully changed permissions of the script");

    let script_str = script_path
        .to_str()
        .ok_or_else(|| eyre!("Failed to convert script path {:?} to string", script_path))?;

    debug!("Executing script: {}", script_str);

    let execute = Command::new("sh")
        .arg("-c")
        .arg(script_str)
        .status()
        .map_err(|e| eyre!("Failed to execute script {:?}: {:?}", script_str, e))?;

    if !execute.success() {
        return Err(eyre!("Script execution failed with status: {:?}", execute));
    }

    Ok(())
}
