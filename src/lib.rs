mod models;
mod command;
mod converter;
mod storage;
mod config;

use std::error::Error;

use clap::Parser;

use crate::command::{Cli, Command};

use crate::storage::{
    deserialize_palette, deserialize_palettes, read_file, serialize_palettes, write_file,
};


pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Convert { from, to, format, input, output } => todo!(),
        Command::List => handle_list(),
        Command::Show {name} => todo!(),
    }
}

//fn handle_convert(from: &str, to: &str, file: &str) -> Result<(), Box<dyn Error>> {
//    // Fetch available palettes
//    let s = read_file("palettes.toml")?;
//    let palettes = deserialize_palettes(&s)?;
//
//    // Fetch the file to convert
//    let conv_file = read_file(file)?;
//
//    // Fetch the two palettes
//    let from_p = palettes
//        .palette_collection
//        .iter()
//        .find(|p| p.name == from)
//        .ok_or_else(|| format!("Couldn't find a palette with name: {}", from))?;
//    let to_p = palettes
//        .palette_collection
//        .iter()
//        .find(|p| p.name == to)
//        .ok_or_else(|| format!("Couldn't find a palette with name: {}", to))?;
//
//    let new_file = replace_colors(from_p, to_p, &conv_file)?;
//
//    // FIX: Doesn't handle the file extension atm. So name will be something like
//    // theme2.toml_solarized
//    let new_file_name = format!("{}_{}", file, to);
//    write_file(&new_file_name, &new_file)?;
//    Ok(())
//}

fn handle_list() -> Result<(), Box<dyn Error>> {
    let s = read_file("palettes.toml")?;
    let palettes = deserialize_palettes(&s)?;
    println!("{palettes:?}");
    Ok(())
}
