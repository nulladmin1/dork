use anyhow::Result;
use clap::{Parser, Subcommand};
use env_logger::Builder;
use log::LevelFilter;

use dork::dork::Dork;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let dork = Dork::new()?;

    let filter_level = match cli.verbose {
        0 => LevelFilter::Warn,
        _ => LevelFilter::Info,
    };

    Builder::new().filter_level(filter_level).init();

    if let Some(command) = &cli.commands {
        match command {
            Commands::Sort => dork.by_type()?,
            Commands::Search { action } => match action {
                Search::Recent => dork.recent()?,
            },
        }
    }

    Ok(())
}
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Sort,
    Search {
        #[command(subcommand)]
        action: Search,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum Search {
    Recent,
}
