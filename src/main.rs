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


fn main() -> Result<()> {

    // Get the arguments from the command line
    let args = Cli::from_args();

    is_brew_available().with_context(|| format!("Unable to find homebrew on the system"))?;

    is_binary_supported(&args.tool.as_str()).with_context(|| format!("Illegal binary"))?;

    // Execute a brew command to retrieve all the available formulaes
    let command_required_version = Command::new("brew")
        .arg("info")
        .arg("--json")
        .arg("--installed")
        .output()
        .with_context(|| format!("Unable to find the brew binary."))?;

    // Check the success of the command
    if !command_required_version.status.success() {
        return Err(anyhow!("brew info --json --installed responded with error code."));
    }

    // Convert the command output to a String
    let file = String::from_utf8(command_required_version.stdout)
        .with_context(|| format!("Error while parsing stdout to utf-8 string"))?;

    // Parse the json output from brew command output to internal Formulae struct
    let json: Vec<Formulae> = serde_json::from_str(&file)
        .with_context(|| format!("Unable to parse json"))?;
    println!("Parsed json structure ==> {:?}", json);

    // Find all the occurrence of the formulae specified by the user
    let mut selected_binary_formulaes: Vec<&Formulae> = vec![];
    for formulae in json.iter() {
        if formulae.name == args.tool {
            selected_binary_formulaes.push(formulae);
        }
    }

    // Check if the required formulae is present on the system
    // Check if there is actually a formulae linked
    // Unlink the current version of the tool
    // Link the new formulae required version



    let mut tool_cli = Command::new(&args.tool);
    tool_cli.arg("--version");
    let out = tool_cli.output().with_context(|| format!("Unable to find the specified binary"))?;

    if out.status.success() {
        println!("Command ok");
    }

    // Display arguments on standard out
    println!("Arg 1 {}", &args.tool);
    println!("Arg 1 {}", &args.version);

    //let str_out = str::from_utf8(&out.stdout).with_context(|| format!("Impossible to parse UTF-8 sequence"))?;

    //println!("Arg 1 {}", str_out);

    Ok(())
}
