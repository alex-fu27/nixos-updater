use crate::nix::{BuildOutput, StorePathError};
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

pub struct Daemon;

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

}

pub fn debug_main() {
    let path = Daemon::new().build();
    println!("out path: {:?}", path);
}

