use std::process::Command;

use anyhow::{anyhow, Context, Result};
use structopt::StructOpt;

use functions::{
    is_binary_supported,
    is_brew_available,
};
use structures::{Cli, Formulae};

mod structures;
mod functions;

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

    // println!("Available versions of {} : {:?}", args.tool, selected_binary_formulaes);


    // Check if the required version of the formulae is available
    let required_version = lenient_semver::parse(args.version.as_str()).unwrap();

    // Retrieve the formulae that the user want ton link
    let mut required_formulae: Option<&Formulae> = None;
    for f in selected_binary_formulaes.iter() {
        let formulae_version = lenient_semver::parse(f.versions.stable.as_str()).unwrap();

        if required_version.major == formulae_version.major && required_version.minor == formulae_version.minor {
            required_formulae = Some(f);
        }
    }

    // Retrieve the actually linked formulae
    let mut actually_linked_formulae: Option<&Formulae> = None;
    for formulae in selected_binary_formulaes.iter() {
        if formulae.linked_keg.is_some() {
            actually_linked_formulae = Some(formulae);
        }
    }

    if required_formulae.is_none() {
        return Err(anyhow!("Required formulae version not found : {}@{}", args.tool, args.version));
    }

    // Link the required version
    if actually_linked_formulae.is_none() {
        // No version is actually linked... link the required version
        println!("The formulae {} version {} will be linkd.", args.tool, args.version);
    } else {
        // If the required version is the version actually linked, do nothing
        if actually_linked_formulae.unwrap().linked_keg == required_formulae.unwrap().linked_keg {
            println!("The formulae version you vant to link is actually linked.")
        } else {
            // Normal case, unlink actually linked and link the new formulae
            println!("Unlinking {} version {}, linking required {} version {}",
                     actually_linked_formulae.unwrap().name,
                     actually_linked_formulae.unwrap().versions.stable,
                     args.tool,
                     required_formulae.unwrap().versions.stable
            )
        }
    }

    println!("Formulae correctly linked !");

    Ok(())
}
