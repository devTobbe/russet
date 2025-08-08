use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "TEST")]
#[command(about = "A palette swapping cli application" , long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Convert a file from one palette to another, available in verison one
    Convert { from: String, to: String },
    /// List the currently available palettes
    List,
    /// Add a palette
    Add { name: String, contents: String },
    /// Delete a palette
    Delete { name: String },
    /// Edit a palette
    Edit { name: String, contents: String },
}
