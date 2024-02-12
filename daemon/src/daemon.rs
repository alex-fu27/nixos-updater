use crate::nix::{BuildOutput, StorePathError};
use std::process::{Command, Stdio};
use std::io;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
pub enum DaemonError {
    StorePathError(StorePathError),
    IOError(io::Error),
}

impl From<io::Error> for DaemonError {
    fn from(e: io::Error) -> Self {
        Self::IOError(e)
    }
}

impl From<StorePathError> for DaemonError {
    fn from(e: StorePathError) -> Self {
        Self::StorePathError(e)
    }
}

type Result<T> = core::result::Result<T, DaemonError>;

pub struct Daemon {
}

fn read_to_lines<T: io::Read>(o: &mut Option<T>) -> io::Lines<io::BufReader<T>> {
    BufReader::new(o.take().unwrap()).lines()
}

impl Daemon { 
    fn new() -> Self {
        Self {}
    }

    fn enter_evaluating_state(&self) {
        println!("evaluating...");
    }

    fn enter_building_state(&self) {
        println!("building...");
    }

    fn build(&self) -> Result<BuildOutput> {
        let wd = mktemp::Temp::new_dir()?;
        let mut child = Command::new("nix")
            .stdin(Stdio::null())
            .stderr(Stdio::piped())
            .stdout(Stdio::null())
            .current_dir(&wd.as_path())
            .args(["--extra-experimental-features", "nix-command flakes",
                "--log-format", "internal-json", "-vv",
                "build", "nixpkgs#hello"])
            .spawn()?;
        let stderr = read_to_lines(&mut child.stderr);

        for line in stderr.flatten() {
            log::debug!("{}", line);
        }

        child.wait()?;

        Ok(BuildOutput::from_temp(wd)?)
    }
}

pub fn debug_main() {
    let path = Daemon::new().build();
    println!("out path: {:?}", path);
}

