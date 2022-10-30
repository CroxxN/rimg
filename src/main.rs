mod directory_helper;
pub use clap::{Parser, Subcommand};
use std::ffi::OsString;

#[derive(Parser)]
pub struct Rimg {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initalize a rimg repository in the current directory
    Init,

    /// add files to the repository
    /// Hint: use "*" for wildcard
    Add { path: OsString },

    /// commit changes to the repository
    Commit {
        #[arg(value_name = "message", long = "message", short = 'm')]
        message: OsString,
    },
}

fn main() {
    let commands = Rimg::parse();

    match commands.command {
        Commands::Init => {
            println!("Found init")
        }
        _ => {
            println!("Idk")
        }
    }
}
