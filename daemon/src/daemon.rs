use std::io;
use std::io::{BufReader, BufRead};

pub struct Daemon;

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
}

