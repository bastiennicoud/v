use std::process::Command;

use anyhow::{anyhow, Context, Result};

// Check if the brew command is available on the system
pub fn is_brew_available() -> Result<()> {
    let command_brew = Command::new("brew")
        .arg("--help")
        .output()
        .with_context(|| "Impossible to launch the brew binary on the system")?;

    if !command_brew.status.success() {
        return Err(anyhow!(
            "Brew found, but responded with an error code {:?}",
            command_brew.status
        ));
    }
    Ok(())
}

// Execute homebrew link
pub fn homebrew_link(formulae_name: &str) -> Result<()> {
    let command_brew = Command::new("brew")
        .arg("link")
        .arg(formulae_name)
        .arg("--overwrite")
        .arg("--force")
        .output()
        .with_context(|| "Unable to find the brew binary.")?;

    if !command_brew.status.success() {
        return Err(anyhow!(
            "Brew found, but responded with an error code {:?}",
            String::from_utf8(command_brew.stdout)
        ));
    }
    Ok(())
}

// Execute homebrew link
pub fn homebrew_unlink(formulae_name: &str) -> Result<()> {
    let command_brew = Command::new("brew")
        .arg("unlink")
        .arg(formulae_name)
        .output()
        .with_context(|| "Unable to find the brew binary.")?;

    if !command_brew.status.success() {
        return Err(anyhow!(
            "Brew found, but responded with an error code {:?}",
            String::from_utf8(command_brew.stdout)
        ));
    }
    Ok(())
}
