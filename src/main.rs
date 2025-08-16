mod command;
mod palette;
mod palette_utils;
mod storage;

use std::error::Error;

use clap::Parser;

use crate::command::{Cli, Command};

use crate::palette_utils::replace_colors;
use crate::storage::{
    deserialize_palette, deserialize_palettes, read_file, serialize_palettes, write_file,
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = read_file("");
    let cli = Cli::parse();

    match &cli.command {
        Command::Convert { from, to, file } => handle_convert(from, to, file),
        Command::List => handle_list(),
        Command::Add { contents } => handle_add(contents),
        Command::Delete { name } => handle_delete(name),
        Command::Edit { name, contents } => {
            todo!()
        }
    }
}

fn handle_convert(from: &str, to: &str, file: &str) -> Result<(), Box<dyn Error>> {
    // Fetch available palettes
    let s = read_file("palettes.toml")?;
    let palettes = deserialize_palettes(&s)?;

    // Fetch the file to convert
    let conv_file = read_file(file)?;

    // Fetch the two palettes
    let from_p = palettes
        .palette_collection
        .iter()
        .find(|p| p.name == from)
        .ok_or_else(|| format!("Couldn't find a palette with name: {}", from))?;
    let to_p = palettes
        .palette_collection
        .iter()
        .find(|p| p.name == to)
        .ok_or_else(|| format!("Couldn't find a palette with name: {}", to))?;

    let new_file = replace_colors(from_p, to_p, &conv_file)?;

    // FIX: Doesn't handle the file extension atm. So name will be something like
    // theme2.toml_solarized
    let new_file_name = format!("{}_{}", file, to);
    write_file(&new_file_name, &new_file)?;
    Ok(())
}

fn handle_list() -> Result<(), Box<dyn Error>> {
    let s = read_file("palettes.toml")?;
    let palettes = deserialize_palettes(&s)?;
    println!("{palettes:?}");
    Ok(())
}

fn handle_add(contents: &str) -> Result<(), Box<dyn Error>> {
    // Fetch available palettes
    let s = read_file("palettes.toml")?;
    let mut palettes = deserialize_palettes(&s)?;

    let deser_p = deserialize_palette(contents)?;
    palettes.palette_collection.push(deser_p);

    let ser_p = serialize_palettes(&palettes)?;
    write_file("palettes.toml", &ser_p)?;

    Ok(())
}

// FIX: Duplicate names is unhandled right now, remeber to handle these cases later on.
fn handle_delete(del_name: &str) -> Result<(), Box<dyn Error>> {
    let s = read_file("palettes.toml")?;
    let mut palettes = deserialize_palettes(&s)?;
    palettes.palette_collection.retain(|p| p.name != del_name);
    let ser_p = serialize_palettes(&palettes)?;
    write_file("palettes.toml", &ser_p)?;
    Ok(())
}
