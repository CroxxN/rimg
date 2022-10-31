mod create_repo;
use clap::{Parser, Subcommand};
use create_repo::CreateRimg;

#[derive(Parser)]
pub struct Rimg {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initalize a rimg repository in the current directory
    Init { exclude_dir: Option<Vec<String>> },

    /// add files to the repository
    /// Hint: use "*" for wildcard
    Add { path: Vec<String> },

    /// commit changes to the repository
    Commit {
        #[arg(value_name = "message", long = "message", short = 'm')]
        message: String,
    },
}

impl Rimg {
    fn init() {
        CreateRimg::create_dir();
    }
    pub fn run(&self) {
        match &self.command {
            Commands::Init { exclude_dir } => {
                Self::init();
                if let Some(options) = exclude_dir {
                    let option = CreateRimg::new(options.to_owned());
                    if let Err(err) = option.walk_dir() {
                        println!("{:?}", err)
                    }
                }
            }
            Commands::Add { path } => {
                println!("{:?}", path);
                todo!();
            }
            _ => {
                println!("Idk")
            }
        }
    }
}

fn main() {
    let commands = Rimg::parse();

    commands.run();
}
