use std::path::PathBuf;

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

pub fn print_config() {
    let cfg_path: PathBuf = confy::get_configuration_file_path("surexe", "config").unwrap();
    println!("Config path: {}", cfg_path.display());
    println!("APIKey: {}", load_config().unwrap().api_key);
} 