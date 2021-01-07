use structopt::StructOpt;
use anyhow::{Context, Result};
use std::process::Command;
use std::str;

// Struct to define required cli arguments
#[derive(StructOpt)]
#[structopt(name = "Version manager", about = "Simple version switcher")]
struct Cli {
    tool: String,
    version: String
}

fn main() -> Result<()> {

    // Get the arguments from the command line
    let args = Cli::from_args();

    // List of supported binaries
    let bins = vec!["php", "node"];

    let mut tool_cli = Command::new(&args.tool);
    tool_cli.arg("--version");
    let out = tool_cli.output().with_context(|| format!("Unable to find the specified binary"))?;

    // Display arguments on standard out
    println!("Arg 1 {}", &args.tool);
    println!("Arg 1 {}", &args.version);

    let str_out = str::from_utf8(&out.stdout).with_context(|| format!("Impossible to parse UTF-8 sequence"))?;

    println!("Arg 1 {}", str_out);

    Ok(())
}
