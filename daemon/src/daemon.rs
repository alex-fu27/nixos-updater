use std::io;
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::errors::*;
use crate::nix::*;
use std::sync::mpsc;
use std::pin::Pin;
use futures::Future;
use futures::task::{Context, Poll};
use tokio::task::JoinHandle;

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

#[derive(Debug)]
pub enum UpgradeState {
    UpdatingInputs,
    BuildingOutput,
    RequiresSwitch,
    SwitchingBoot,
    RequiresReboot,
    SwitchingConfiguration,
    Done
}

pub struct UpgradeProcessInfo {
    out_queue: Option<mpsc::Receiver<UpgradeState>>,
    result: Option<JoinHandle<Result<(), UpgradeError>>>,
}

pub struct UpgradeProcess {
    input: Box<dyn Manageable + Send>,
    profile: Profile,
}

impl UpgradeProcess {
    pub fn for_flake(flake: FlakeConfig) -> Self {
        Self {
            input: Box::new(flake),
            profile: Profile::system(),
        }
    }

    fn compute_required_action(&self, new: &BuildOutput) -> Result<UpgradeNeeds, UpgradeError> {
        let sys = self.profile.get_current()?;
        Ok(UpgradeNeeds::compare(&sys, &new.path)?)
    }

    fn do_switch(&self, out: BuildOutput) -> Result<(), UpgradeError> {
        let binary = out.path.subpath("bin/switch-to-configuration");
        let map = |e| UpgradeError::SwitchFailed(Some(e));
        let mut chld = Command::new(binary).arg("switch").spawn().map_err(map)?;
        let out = chld.wait().map_err(map)?;
        if ! out.success() {
            return Err(UpgradeError::SwitchFailed(None));
        }
        Ok(())
    }

    fn do_reboot(&self, out: BuildOutput) -> Result<(), UpgradeError> {
        todo!();
    }

    pub fn run(self) -> UpgradeProcessInfo {
        let (out_tx, out_queue) = mpsc::channel();

        let result = tokio::spawn(async move {
            out_tx.send(UpgradeState::UpdatingInputs).unwrap();
            self.input.update()?;
            out_tx.send(UpgradeState::BuildingOutput).unwrap();
            let out = self.input.build()?;
            let action = self.compute_required_action(&out)?;
            match action {
                UpgradeNeeds::None => Ok::<(), UpgradeError>(()),
                UpgradeNeeds::Switch => self.do_switch(out),
                UpgradeNeeds::Reboot => self.do_reboot(out),
            }
        });

        UpgradeProcessInfo {
            out_queue: Some(out_queue),
            result: Some(result),
        }
    }
}

pub async fn debug_main() -> anyhow::Result<()> {
    let d = UpgradeProcess::for_flake(FlakeConfig::from_url_and_config_name("/home/alex/.config/nixpkgs", "flink"));

    let mut r = d.run();

    for i in r.out_queue.take().unwrap() {
        println!("got {:?}", i);
    }

    println!("result {:?}", r.result.take().unwrap().await.unwrap());

    Ok(())
}

