use clap::{Parser, Subcommand};

#[derive(Parser)]
// le informazioni necessarie verranno prese dal Cargo.toml
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct App {
    // sottocommando
    #[command(subcommand)]
    command: Commands,
}

impl App {
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
