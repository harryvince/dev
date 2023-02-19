mod api;
mod args;
use clap::Parser;

fn main() -> Result<(), std::io::Error> {
    let cli = args::Cli::parse();

    match &cli.command {
        Some(args::Commands::Init(input)) => {
            match &input.command {
                Some(args::InitSubcommand::Sys) => {
                    let pm = api::system::find_package_manager()?;
                    println!("Package Manager: {}", pm.as_str());
                }
                None => {
                    println!("Please provide valid args, --help to see available args");
                }
            }
        }
        None => {
            println!("Please provide valid args, --help to see available args");
        }
    }

    Ok(())
}
