use clap::{Parser, Subcommand, Args};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Intializes Dev Tools
    Init(InitCommands),
}

#[derive(Debug, Args)]
pub struct InitCommands {
    #[clap(subcommand)]
    pub command: Option<InitSubcommand>,
}

#[derive(Debug, Subcommand)]
pub enum InitSubcommand {
    /// Initializes a system with the base requirements
    Sys,
}
