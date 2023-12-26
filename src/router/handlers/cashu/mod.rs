use serde::{Deserialize, Serialize};

pub mod check;
pub mod info;
pub mod keys;
pub mod keysets;
pub mod melt;
pub mod mint;
pub mod swap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SupportedUnit {
    Msat,
    Sat,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Method {
    Bolt11,
    Onchain,
}
