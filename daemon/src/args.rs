use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Command {
    Daemon,
    Status,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    #[arg(short, long, action=clap::ArgAction::Count)]
    pub verbose: u8,
    #[command(subcommand)]
    pub command: Command,
}

pub fn parse() -> Args {
    Args::parse()
}

