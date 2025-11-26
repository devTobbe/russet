mod command;
mod converter;
mod models;
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
        Command::Convert {
            from,
            to,
            format,
            input,
            output,
        } => {
            let conf = build_args(from, to, format, input, output)?;
            handle_convert(conf)
        }
        Command::List => handle_list(),
        Command::Show { name } => todo!(),
    }
}

fn handle_list() -> Result<(), Box<dyn Error>> {
    let s = read_file("palettes.toml")?;
    let palettes = deserialize_palettes(&s)?;
    println!("{palettes:?}");
    Ok(())
}

fn build_args(
    from: &str,
    to: &str,
    format: &str,
    input: &str,
    output: &str,
) -> Result<Config, Box<dyn Error>> {
    let builder = ConfigBuilder::new();

    builder
        .from(from)
        .to(to)
        .format(format)
        .input(input)
        .output(output)
        .build()
}

fn apply_format(format: &ColorFormat, col: &mut Palette) {
    match format {
        ColorFormat::Rgb => {
            col.convert_all_to_rgb();
        }
        ColorFormat::Hsl => {
            col.convert_all_to_hsl();
        }
    }
}

fn handle_convert(conf: Config) -> Result<(), Box<dyn Error>> {
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
    let mut fromp = input_deser
        .clone_palette(conf.from())
        .ok_or("From palette not found")?;
    let mut to = input_deser
        .clone_palette(conf.to())
        .ok_or("To palette not found")?;
    //let format = ColorFormat::identify(conf.format());

    // Not the best since this will happen every time, better if it only performs apply if format
    // isn't already the right format.
    //let _ = apply_format(&format, &mut fromp);
    //let _ = apply_format(&format, &mut to);

    // Converter Generate Replacement Rules
    // Apply Replacements
    //
    // Save File

    todo!()
}

fn handle_show() -> Result<(), Box<dyn Error>> {
    todo!()
}
