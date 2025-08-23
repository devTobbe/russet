use clap::{Parser, Subcommand};

// TODO: Add posibility for flags for better user ergonomics
#[derive(Parser, Debug)]
#[command(name = "TEST")]
#[command(version, about = "A palette swapping cli application" , long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

// TODO: Add possibility of selecting output file name (and location I suppose)
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Convert a file from one palette to another, available in verison one
    Convert {
        from: String,
        to: String,
        file: String,
    },
    /// List the currently available palettes
    List,
    /// Add a palette
    Add { contents: String },
    /// Delete a palette
    Delete { name: String },
    /// Edit a palette
    Edit { name: String, contents: String },
}
