mod command;
mod palette;
mod palette_utils;
mod storage;

use std::error::Error;

use clap::Parser;

use crate::command::{Cli, Command};

use crate::storage::{deserialize_palettes, read_file};

fn main() -> Result<(), Box<dyn Error>> {
    let file = read_file("");
    let cli = Cli::parse();

    match &cli.command {
        Command::Convert { from, to } => {
            todo!()
        }
        Command::List => handle_list(),
        Command::Add { name, contents } => {
            todo!()
        }
        Command::Delete { name } => {
            todo!()
        }
        Command::Edit { name, contents } => {
            todo!()
        }
    }
}

fn handle_list() -> Result<(), Box<dyn Error>> {
    let s = read_file("palettes.toml")?;
    let palettes = deserialize_palettes(&s)?;
    println!("{palettes:?}");
    Ok(())
}
