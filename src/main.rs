mod api;
use clap::{Parser, Subcommand, Args};

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Installs required base features for a system
    InitSystem,
}


fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::InitSystem) => {
            let pm = api::system::find_package_manager()?;
            println!("{}", pm.as_str());
        }
        None => {
            println!("Please provide args for the CLI");
        }
    }
    Ok(())
}
