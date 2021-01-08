mod structures;
mod functions;

use structures::{Cli, Formulae};
use functions::{is_binary_supported};

use structopt::StructOpt;
use anyhow::{Context, Result, anyhow};
use std::process::Command;
use std::fs;


fn main() -> Result<()> {

    // Get the arguments from the command line
    let args = Cli::from_args();

    is_binary_supported(&args.tool.as_str()).with_context(|| format!("Illegal binary"))?;



    // Prepare a brew command to check if the required version of the formulae is present
    let command_required_version = Command::new("brew")
        .arg("list")
        .arg(format!("{}@{}", args.tool, args.version))
        .output()
        .with_context(|| format!("Unable to find the brew binary."))?;

    // Check the success of the command
    if command_required_version.status.success() {
        println!("The required version of {} was found : {}", &args.tool, &args.version);
    } else {
        return Err(anyhow!("The desired formulae version is not installed."));
    }

    // Test serde json
    let file = "./brew.json";
    let data = fs::read_to_string(file).with_context(|| format!("Unable to read file"))?;
    // println!("Json string ==> {}", data);
    let json: Vec<Formulae> = serde_json::from_str(&data).with_context(|| format!("Unable to parse json"))?;
    println!("Parsed json structure ==> {:?}", json);

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
