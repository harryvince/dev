mod api;
mod args;
use clap::Parser;

fn main() -> Result<(), std::io::Error> {
    let cli = args::Cli::parse();

    match &cli.command {
        Some(args::Commands::Init(input)) => {
            match &input.command {
                Some(args::InitSubcommand::Sys) => {
                    println!("We're going to ask you for sudo access, this is just needed for some setup");
                    api::system::gain_sudo()?;
                    let pm = api::system::find_package_manager()?;
                    println!("Package Manager found: {}", pm.as_str());
                    pm.install_packages(vec!["test", "package"])?;
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
