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
         /// The palette to switch from
        #[arg(short, long)]
        from: String,
         /// The palette to switch to
        #[arg(short, long)]
        to: String,
         /// The format of the colors on the target file, supported are: RGB, HEX, HSL
        #[arg(short, long)]
        format: String,
         /// Input file
        #[arg(short, long)]
        input: String,
         /// Specify an output file
        #[arg(short, long)]
        output: String,
    },
    /// List the currently available palettes
    List,
    /// Show a specigic palette
    Show {
        name: String
    }
}
