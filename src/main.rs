mod command;
mod palette;
mod palette_utils;
mod storage;

use std::error::Error;

use clap::Parser;

use crate::command::{Cli, Command};

use crate::storage::read_file;

fn main() -> Result<(), Box<dyn Error>> {
    let file = read_file("");
    let cli = Cli::parse();

    match cli.command {
        Command::Convert { from, to } => {}
        Command::List => {}
        Command::Add { name, contents } => {}
        Command::Delete { name } => {}
        Command::Edit { name, contents } => {}
    }
}
