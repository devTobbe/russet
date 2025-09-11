mod models;
mod command;
mod converter;
mod storage;

use std::error::Error;

use clap::Parser;

use crate::command::{Cli, Command};

use crate::models::color::ColorFormat;
use crate::models::config::{Config, ConfigBuilder};
use crate::models::palette::Palette;
use crate::storage::{
    deserialize_palette, deserialize_palettes, read_file, serialize_palettes, write_file,
};


pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Convert { from, to, format, input, output } => {
            let conf = build_args(from, to, format, input, output)?;
            handle_convert(conf)
        },
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

fn build_args(from: &str, to: &str, format: &str, input: &str, output: &str) -> Result<Config, Box<dyn Error>> {
    let builder = ConfigBuilder::new();

    builder
        .from(from)
        .to(to)
        .format(format)
        .input(input)
        .output(output)
        .build()
}

fn handle_convert(conf : Config) -> Result<(), Box<dyn Error>>{
    // NOTE: Order of actions:  
    // 1. Fetch From and To from Input
    // 2. Fetch format
    // 3. If format differs, convert
    // 4. Converter ->
    // 5.   Generate Replacement Rules
    // 6.   Apply Replacements
    // 7. Wrtie to Output

    let input = read_file("palettes.toml")?;
    let input_deser = deserialize_palettes(&input)?;
    // TODO: Maybe implement this a bit better for a palette collection
    let fromp : &mut models::palette::Palette = &mut input_deser.palette_collection().iter().find(|p| p.get_name() == conf.from()).unwrap();
    let top : &mut models::palette::Palette = &mut input_deser.palette_collection().iter().find(|p| p.get_name() == conf.to()).unwrap();
    let format = ColorFormat::identify(conf.format());
    match format {
        ColorFormat::Rgb => {fromp.convert_all_to_rgb();top.convert_all_to_rgb();}
        ColorFormat::Hsl => {fromp.convert_all_to_hsl();top.convert_all_to_hsl();}
    }

    todo!()
}

fn handle_show() -> Result<(), Box<dyn Error>>{
    todo!()
}
