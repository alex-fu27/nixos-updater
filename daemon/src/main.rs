mod dbus_daemon;
mod client;
pub mod args;
pub mod consts;

use log::debug;
use args::{Args, Command};

fn setup_logging(verbosity: u8) {
    stderrlog::new()
        .module(module_path!())
        // +1 to make warn visible by default
        .verbosity(usize::from(verbosity) + 1)
        .init()
        .unwrap();
}

fn handle_client_commandline(args: &Args) -> anyhow::Result<()> {
    let client = client::Client::new()?;
    let res = match args.command {
        Command::Status => client.print_status(),
        Command::BuildUpdate => client.build_update(),
        Command::Daemon => unreachable!(),
    };
    Ok(res?)
}

fn main() -> anyhow::Result<()> {
    let args = args::parse();

    setup_logging(args.verbose);
    debug!("Arguments: {:?}", args);

    match args.command {
        Command::Daemon =>
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(dbus_daemon::main()),
        _ => handle_client_commandline(&args),
    }
}
