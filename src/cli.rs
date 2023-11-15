use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "pgn_parser - is a tool that allows you to parse chess games in PGN format.\nAuthor - Illia Shevchyk <illia.shevchyk@ukma.edu.ua>"
)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Command which parse PGN
    Parse {
        ///provide input file
        #[arg(short, long, value_name = "INPUT")]
        input: Option<PathBuf>,
        ///provide output file
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,
    },
}
