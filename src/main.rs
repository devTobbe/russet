mod command;
mod palette;
mod palette_utils;
mod storage;

use std::error::Error;

use clap::Parser;

use crate::command::{Cli, Command};

use crate::storage::{deserialize_palettes, read_file, serialize_palettes, write_file};

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
        Command::Delete { name } => {handle_delete(name)}
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

// FIX: Duplicate names is unhandled right now, remeber to handle these cases later on.
fn handle_delete(del_name: &str) -> Result<(), Box<dyn Error>> {
    let s = read_file("palettes.toml")?;
    let mut palettes = deserialize_palettes(&s)?;
    palettes.palette_collection.retain(|p| p.name != del_name);
    let deser_p = serialize_palettes(&palettes)?;
    write_file("palettes.toml", &deser_p)?;
    Ok(())
}
