use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

impl Args {
    pub fn command(&self) -> &Commands {
        &self.command
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Print,
    Search { name: String },
    Add { name: String },
    Remove { name: String },
    Update { name: String },
}
