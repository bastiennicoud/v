use serde::{Deserialize, Serialize};

/// Represent a brew formulae
/// Conserves only the formulae name and the keg version linked
#[derive(Serialize, Deserialize, Debug)]
pub struct Formulae {
    pub name: String,
    pub version: Version, // formulae package version
    pub linked_keg: Option<String>, // If linked, version linked
}

/// Represent the nested formulae version
#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub stable: String,
}
