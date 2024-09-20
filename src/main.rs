use std::path::PathBuf;

mod config;
mod surexe;

fn main() {
    let config_path: PathBuf = confy::get_configuration_file_path("surexe", "config").unwrap();
    println!("Config path: {}", config_path.display());
    println!("APIKey: {}", config::load_config().unwrap().api_key);
}