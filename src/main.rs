extern crate pest;
extern crate pest_derive;

use clap::Parser;
use crate::cli::{Cli, Commands};
use pgn_parser::parse_pgn;
mod cli;

pub fn main() {
    let cli = Cli::parse();


    match &cli.command {
        Some(Commands::Parse { input, output }) => {
            if let Some(path_to_file) = input {
                let file_content = std::fs::read_to_string(path_to_file).expect("Unable to read file");
                let parsed_game = parse_pgn(&file_content).expect("TODO: panic message");
                if let Some(output_path) = output {
                    std::fs::write(output_path, parsed_game.to_string()).expect("Unable to write result");
                } else {
                    println!("{}", parsed_game.to_string());
                }
            } else {
                println!("Missing path to file!");
            }
        }
        None => {}
    }
}