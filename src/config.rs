use confy;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let cfg: Config = confy::load("surexe", "config").unwrap();
    Ok(cfg)
}