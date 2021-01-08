use structopt::StructOpt;
use serde::{Serialize, Deserialize};

// Struct to define required cli arguments
#[derive(StructOpt)]
#[structopt(name = "Version manager", about = "Simple version switcher")]
pub struct Cli {
    pub tool: String,
    pub version: String
}

// Represent brew installed formulae list
// Conserves only the formulae name and the keg version linked
#[derive(Serialize, Deserialize, Debug)]
pub struct Formulae {
    pub name: String,
    pub linked_keg: Option<String>
}
