use std::io;
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};
use crate::errors::*;
use crate::nix::*;

trait Manageable: Updateable + Buildable {}
impl<T: Updateable + Buildable> Manageable for T {}

#[derive(Debug)]
pub enum UpgradeNeeds {
    None,
    Switch,
    Reboot,
}

impl UpgradeNeeds {
    fn read_system_file_link(base: &StorePath, suffix: &str) -> Result<StorePath, StorePathError> {
        let mut pb: PathBuf = base.into();
        pb.push(suffix);
        todo!();
    }

    fn sublink_eq(from: &StorePath, to: &StorePath, sub: &str) -> Result<bool, StorePathError> {
        let p1 = StorePath::new(&from.subpath(sub))?;
        let p2 = StorePath::new(&to.subpath(sub))?;
        Ok(p1 == p2)
    }

    pub fn compare(from: &StorePath, to: &StorePath) -> Result<Self, StorePathError> {
        if from == to {
            return Ok(UpgradeNeeds::None);
        }
        if Self::sublink_eq(from, to, "initrd")?
                && Self::sublink_eq(from, to, "kernel")?
                && Self::sublink_eq(from, to, "kernel-modules")? {
            return Ok(UpgradeNeeds::Switch);
        }
        Ok(UpgradeNeeds::Reboot)
    }
}

pub enum DaemonState {
    Idle,
    UpdatingInputs,
    BuildingOutput,
    RequiresSwitch,
    SwitchingBoot,
    RequiresReboot,
    SwitchingConfiguration,
}

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

    pub fn full_upgrade(&self) -> Result<Option<(BuildOutput, UpgradeNeeds)>, UpgradeError> {
        let out = self.update_and_build()?;
        let sys = self.profile.get_current()?;
        log::debug!("current: {}, new: {}", sys, out.path);
        let un = UpgradeNeeds::compare(&sys, &out.path)?;
        Ok(match un {
            UpgradeNeeds::None => None,
            un => Some((out, un)),
        })
    }
}

pub fn debug_main() -> anyhow::Result<()> {
    let upgr = Daemon::for_flake(FlakeConfig::from_url_and_config_name("/home/alex/.config/nixpkgs", "flink")).full_upgrade()?;
    println!("result: {:?}", upgr);
    Ok(())
}

