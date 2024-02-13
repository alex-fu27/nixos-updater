use std::io;
use std::io::{BufReader, BufRead};
use crate::errors::*;
use crate::nix::{Profile, FlakeConfig, Updateable, Buildable, BuildOutput};

trait Manageable: Updateable + Buildable {}
impl<T: Updateable + Buildable> Manageable for T {}

pub struct Daemon {
    input: Box<dyn Manageable>,
    profile: Profile,
}

impl Daemon {
    pub fn for_flake(flake: FlakeConfig) -> Self {
        Self {
            input: Box::new(flake),
            profile: Profile::system(),
        }
    }

    pub fn update_and_build(&self) -> Result<BuildOutput, UpgradeError> {
        log::info!("updating inputs...");
        if self.input.update()? {
            log::info!("inputs changed");   
        } else {
            log::info!("inputs unchanged");   
        }
        log::info!("building output...");
        Ok(self.input.build()?)
    }

    pub fn full_upgrade(&self) -> Result<Option<BuildOutput>, UpgradeError> {
        let out = self.update_and_build()?;
        let sys = self.profile.get_current()?;
        log::debug!("current: {}, new: {}", sys, out.path);
        if out.path != sys {
            log::info!("need update!");
            return Ok(Some(out));
        }
        Ok(None)
    }
}

pub fn debug_main() -> anyhow::Result<()> {
    let upgr = Daemon::for_flake(FlakeConfig::from_url_and_config_name("/home/alex/.config/nixpkgs", "flink")).full_upgrade()?;
    match upgr {
        None => println!("no upgrade available"),
        Some(output) => println!("upgrade built to {}", &output.path),
    }
    Ok(())
}

