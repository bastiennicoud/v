use anyhow::{Result, anyhow, Context};
use std::process::Command;

pub fn is_brew_available() -> Result<()> {
    let command_brew = Command::new("brew")
        .arg("--help")
        .output()
        .with_context(|| format!("Impossible to launch the brew binary on the system"))?;

    if !command_brew.status.success() {
        return Err(anyhow!("Brew found, but responded with an error code {:?}", command_brew.status));
    }
    Ok(())
}

// Check if the binary is supported by the tool
pub fn is_binary_supported(binary : &&str) -> Result<()> {
    // List of supported binaries
    let bins = vec!["php"];

    // Check if the user entry is supported
    if !bins.contains(binary) {
        return Err(anyhow!("This tool is not supported, supported binaries : {:?}", bins));
    }
    Ok(())
}

// Execute homebrew link
pub fn homebrew_link(formulae_name: &String) -> Result<()> {

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
pub fn homebrew_unlink(formulae_name: &String) -> Result<()> {

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