extern crate pest;
extern crate pest_derive;

use crate::cli::{Cli, Commands};
use clap::Parser;
use pgn_parser::parse_pgn;
mod cli;

pub fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Parse { input, output }) => {
            if let Some(path_to_file) = input {
                let file_content =
                    std::fs::read_to_string(path_to_file).expect("Unable to read file");
                let parsed_game = parse_pgn(&file_content);
                let result: String = match parsed_game {
                    Ok(game) => game.to_string(),
                    Err(e) => e.to_string(),
                };
                if let Some(output_path) = output {
                    std::fs::write(output_path, result).expect("Unable to write result");
                } else {
                    println!("{}", result);
                }
            } else {
                println!("Missing path to file!");
            }
        }
        None => {}
    }
}
