mod api;
use clap::{Parser, Subcommand, Args};

/// Simple program to greet a person
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Intializes Dev Tools
    Init(InitCommands),
}

#[derive(Debug, Args)]
struct InitCommands {
    #[clap(subcommand)]
    command: Option<InitSubcommand>,
}

#[derive(Debug, Subcommand)]
enum InitSubcommand {
    /// Initializes a system with the base requirements
    Sys,
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init(input)) => {
            match &input.command {
               Some(InitSubcommand::Sys) => {
                   let pm = api::system::find_package_manager()?;
                   println!("Package Manager: {}", pm.as_str());
               }
               None => {
                   println!("Please provide valid args, --help to see available args");
               }
            }
        }
        None => {
            println!("No command provided");
        }
    }

    Ok(())
}
