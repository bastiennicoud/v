use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Version manager", about = "Simple version switcher")]
struct Cli {
    tool: String,
    version: String
}

fn main() {

    // Get the arguments from the command line
    let args = Cli::from_args();

}
