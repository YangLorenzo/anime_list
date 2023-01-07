mod app;

use app::App;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::parse();

    match app.command() {
        app::Commands::Print => println!("Print"),
        app::Commands::Search { name } => println!("Search {}", name),
        app::Commands::Add { name } => println!("Add {}", name),
        app::Commands::Remove { name } => println!("Remove {}", name),
        app::Commands::Update { name } => println!("Update {}", name),
    }
    Ok(())
}
