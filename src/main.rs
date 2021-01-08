mod structures;
mod functions;

use structures::{Cli, Formulae};
use functions::{
    is_binary_supported,
    is_brew_available
};

use structopt::StructOpt;
use anyhow::{Context, Result, anyhow};
use std::process::Command;
use lenient_semver;


fn main() -> Result<()> {

    // Get the arguments from the command line
    let args = Cli::from_args();

    is_brew_available()
        .with_context(|| "Unable to find homebrew on the system")?;

    is_binary_supported(&args.tool.as_str())
        .with_context(|| format!("Illegal binary : {}", args.tool))?;

    println!("Checking if required formulae and version are available.");

    // Execute a brew command to retrieve all the available formulaes
    let command_required_version = Command::new("brew")
        .arg("info")
        .arg("--json")
        .arg("--installed")
        .output()
        .with_context(|| "Unable to find the brew binary.")?;

    // Check the success of the command
    if !command_required_version.status.success() {
        return Err(anyhow!("brew info --json --installed responded with error code."));
    }

    // Convert the command output to a String
    let file = String::from_utf8(command_required_version.stdout)
        .with_context(|| "Error while parsing stdout to utf-8 string")?;

    // Parse the json output from brew command output to internal Formulae struct
    let json: Vec<Formulae> = serde_json::from_str(&file)
        .with_context(|| "Unable to parse json")?;

    // Find all the occurrence of the formulae specified by the user
    let mut selected_binary_formulaes: Vec<&Formulae> = vec![];
    for f in json.iter() {

        // Get the name of the formulae without the version suffix
        let name = f.name
            .split('@')
            .nth(0)
            .with_context(|| format!("Error while trying to get the formulae name : {:?}", f))?;

        if name == args.tool {
            selected_binary_formulaes.push(f);
        }
    }

    // Terminate if the required formulae in not installed
    if selected_binary_formulaes.is_empty() {
        return Err(anyhow!("No {} formulae installed on this system.", args.tool));
    }

    println!("Available versions of {} : {:?}", args.tool, selected_binary_formulaes);


    //print!("Test semver {:?}", lenient_semver::parse("8.0"));
    //print!("Test semver {:?}", lenient_semver::parse("8.0.0"));
    // Check if the required version of the formulae is available
    //let required_version = lenient_semver::parse(args.version.as_str())
    //    .with_context(|| format!(
    //        "Impossible to parse the provided version : {} Please check if this is a valid semver.",
    //        args.version))?;

    // for f in selected_binary_formulaes.iter() {
    //     let formulae_version = lenient_semver::parse(f.versions.stable.as_str())
    //         .with_context(|| "Impossible to parse the formulae wersion")?;
    //
    //     if required_version == formulae_version {
    //         println!("Matching version {}", f.versions.stable);
    //     } else {
    //         print!("Not match {}", f.versions.stable);
    //     }
    // }

    // Check if any of the version formulae is linked
    // let mut actually_linked_formulae: Option<&Formulae> = None;
    // for formulae in selected_binary_formulaes.iter() {
    //     if formulae.linked_keg.is_some() {
    //         actually_linked_formulae = Some(formulae);
    //     }
    // }

    // println!("Parsed json structure ==> {:?}", selected_binary_formulaes);

    // Check if the required formulae is present on the system
    // Check if any of the version formulae is linked
    // Check if the actually linked formulae is present
    // Check if there is actually a formulae linked
    // Unlink the current version of the tool
    // Link the new formulae required version



    // let mut tool_cli = Command::new(&args.tool);
    // tool_cli.arg("--version");
    // let out = tool_cli.output().with_context(|| format!("Unable to find the specified binary"))?;
    //
    // if out.status.success() {
    //     println!("Command ok");
    // }

    // Display arguments on standard out
    //println!("Arg 1 {}", args.tool);
    //println!("Arg 1 {}", args.version);

    //let str_out = str::from_utf8(&out.stdout).with_context(|| format!("Impossible to parse UTF-8 sequence"))?;

    //println!("Arg 1 {}", str_out);

    Ok(())
}
