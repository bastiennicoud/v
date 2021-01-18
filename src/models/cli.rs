use structopt::StructOpt;

// Struct to define required cli arguments
#[derive(StructOpt)]
#[structopt(name = "Version manager", about = "Simple version switcher")]
pub struct Cli {
    pub tool: String,
    pub version: String,
}
