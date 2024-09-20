mod config;
mod surexe;

use clap::{Subcommand, Parser};
use surexe::post_gemini;

#[derive(Parser)]
#[clap(name = "surexe", version = "0.1.0", author = "Esurio", about = "Are you sure to execute this command?")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Config {
        #[clap(subcommand)]
        subcommand: ConfigCommands,
    },
    Surexe {
        #[clap(subcommand)]
        subcommand: SurexeCommands,
    }
}

#[derive(Subcommand)]
enum ConfigCommands {
    Show,
}

#[derive(Subcommand)]
enum SurexeCommands {
    Run {
        command: String,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Config { subcommand } => match subcommand {
            ConfigCommands::Show => config::print_config(),
        },
        Commands::Surexe { subcommand } => match subcommand {
            SurexeCommands::Run { command } => {
                let api_key = config::load_config().unwrap().api_key;
                let parts = vec![command.as_str()];
                match post_gemini(parts, &api_key) {
                    Ok(res) => {
                        if let Err(e) = surexe::display_response(&res) {
                            eprintln!("{}", e);
                        }
                    }
                    Err(e) => eprintln!("{}", e),
                }
            }
        },
    }
}