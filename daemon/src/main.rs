mod daemon;
mod client;
pub mod consts;

use log::debug;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Command {
    Daemon,
    Status,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long, action=clap::ArgAction::Count)]
    verbose: u8,
    #[command(subcommand)]
    command: Command,
}

fn setup_logging(verbosity: u8) {
    stderrlog::new()
        .module(module_path!())
        .verbosity(usize::from(verbosity))
        .init()
        .unwrap();
}

fn handle_client_command(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let client = client::Client::new()?;
    let res = match args.command {
        Command::Status => client.print_status(),
        Command::Daemon => unreachable!(),
    };
    Ok(res?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    setup_logging(args.verbose);
    debug!("Arguments: {:?}", args);

    match args.command {
        Command::Daemon =>
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(daemon::main()),
        _ => handle_client_command(&args),
    }
}
