use anyhow::{anyhow, Result};

/// Check if the binary is supported by the tool
///
/// # Examples
///
/// ```
/// use helpers::compatibility::is_binary_supportedk;
/// assert_eq!(is_binary_supported("php"), Ok);
/// ```
pub fn is_binary_supported(binary: &&str) -> Result<()> {
    // List of supported binaries
    let bins = vec!["php", "node"];

    // Check if the user entry is supported
    if !bins.contains(binary) {
        return Err(anyhow!(
            "This tool is not supported, supported binaries : {:?}",
            bins
        ));
    }
    Ok(())
}
