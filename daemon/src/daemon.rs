use std::io;
use std::io::{BufReader, BufRead};
use crate::errors::*;
use crate::nix::{Flake, Updateable, Buildable, BuildOutput};

trait Manageable: Updateable + Buildable {}
impl<T: Updateable + Buildable> Manageable for T {}

pub struct Daemon {
    input: Box<dyn Manageable>,
}

impl Daemon {
    fn for_flake(flake: Flake) -> Self {
        Self { input: Box::new(flake) }
    }

    fn update_and_build(&self) -> Result<Option<BuildOutput>, UpgradeError> {
        println!("updating inputs...");
        if ! self.input.update()? {
            return Ok(None);
        }
        Ok(Some(self.input.build()?))
    }
}

pub fn debug_main() -> anyhow::Result<()> {
    let upgr = Daemon::for_flake(Flake::from_url("/home/alex/.config/nixpkgs#nixosConfigurations.flink.config.system.build.toplevel")).update_and_build()?;
    match upgr {
        None => println!("no upgrade available"),
        Some(output) => println!("upgrade built to {}", &output.path),
    }
    Ok(())
}

