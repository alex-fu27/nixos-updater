mod daemon;
mod client;
pub mod args;
pub mod consts;

use log::debug;
use args::{Args, Command};

fn setup_logging(verbosity: u8) {
    stderrlog::new()
        .module(module_path!())
        .verbosity(usize::from(verbosity))
        .init()
        .unwrap();
}

fn handle_client_commandline(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let client = client::Client::new()?;
    let res = match args.command {
        Command::Status => client.print_status(),
        Command::Daemon => unreachable!(),
    };
    Ok(res?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::parse();

    setup_logging(args.verbose);
    debug!("Arguments: {:?}", args);

    match args.command {
        Command::Daemon =>
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(daemon::main()),
        _ => handle_client_commandline(&args),
    }
}
