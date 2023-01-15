mod anime;
mod args;
mod data;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    match args.command() {
        args::Commands::Print => println!("Print"),
        args::Commands::Search { name } => println!("Search {}", name),
        args::Commands::Add { name } => println!("Add {}", name),
        args::Commands::Remove { name } => println!("Remove {}", name),
        args::Commands::Update { name } => println!("Update {}", name),
    }
}
