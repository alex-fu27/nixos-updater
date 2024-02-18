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
pub enum UpgradeCommand {
    Cancel,
    Switch,
    SetBoot,
    Reboot,
}

#[derive(Debug, PartialEq)]
pub enum UpgradeState {
    UpdatingInputs,
    BuildingOutput,
    RequiresSwitch,
    SwitchingBoot,
    RequiresReboot,
    SwitchingConfiguration,
    Rebooting,
    Done
}

pub struct UpgradeProcessInfo {
    out_queue: Option<mpsc::Receiver<UpgradeState>>,
    in_queue: mpsc::Sender<UpgradeCommand>,
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

    fn exec_switch_to_configuration(&self, out: &BuildOutput, arg: &str) -> Result<(), UpgradeError> {
        let binary = out.path.subpath("bin/switch-to-configuration");
        let mut chld = Command::new(binary).arg(arg).spawn().map_err(UpgradeError::map_switch_io_error)?;
        let out = chld.wait().map_err(UpgradeError::map_switch_io_error)?;
        if ! out.success() {
            return Err(UpgradeError::SwitchFailed(None));
        }
        Ok(())
    }

    fn switch_to(&self, out: &BuildOutput) -> Result<(), UpgradeError> {
        self.exec_switch_to_configuration(out, "switch")
    }

    fn make_boot_default(&self, out: &BuildOutput) -> Result<(), UpgradeError> {
        self.exec_switch_to_configuration(out, "boot")
    }

    fn reboot(&self) -> Result<(), UpgradeError> {
        let out = Command::new("reboot")
            .output().map_err(UpgradeError::map_reboot_failed)?;
        if ! out.status.success() {
            Err(UpgradeError::RebootFailed(String::from_utf8(out.stderr).unwrap()))?;
        }
        Ok(())
    }

    pub fn run(self) -> UpgradeProcessInfo {
        let (out_tx, out_queue) = mpsc::channel();
        let (in_queue, in_rx) = mpsc::channel();

        let result = tokio::spawn(async move {
            out_tx.send(UpgradeState::UpdatingInputs).unwrap();
            self.input.update()?;
            out_tx.send(UpgradeState::BuildingOutput).unwrap();
            let out = self.input.build()?;
            let action = self.compute_required_action(&out)?;
            match action {
                UpgradeNeeds::None => {
                    out_tx.send(UpgradeState::Done).unwrap();
                    return Ok::<(), UpgradeError>(());
                },
                UpgradeNeeds::Switch => {
                    out_tx.send(UpgradeState::RequiresSwitch);
                },
                UpgradeNeeds::Reboot => {
                    out_tx.send(UpgradeState::RequiresReboot);
                },
            }

            let cmd = in_rx.recv().unwrap();
            match cmd {
                UpgradeCommand::Cancel => {
                    out_tx.send(UpgradeState::Done).unwrap();
                    return Err(UpgradeError::Cancelled);
                },
                UpgradeCommand::Switch => {
                    out_tx.send(UpgradeState::SwitchingConfiguration);
                    self.switch_to(&out)?;
                },
                UpgradeCommand::SetBoot => {
                    out_tx.send(UpgradeState::SwitchingBoot);
                    self.make_boot_default(&out)?;
                },
                UpgradeCommand::Reboot => {
                    out_tx.send(UpgradeState::SwitchingBoot);
                    self.make_boot_default(&out)?;
                    out_tx.send(UpgradeState::Rebooting);
                    self.reboot()?;
                },
            }
            out_tx.send(UpgradeState::Done).unwrap();
            Ok(())
        });

        UpgradeProcessInfo {
            out_queue: Some(out_queue),
            in_queue,
            result: Some(result),
        }
    }
}

pub async fn debug_main() -> anyhow::Result<()> {
    let d = UpgradeProcess::for_flake(FlakeConfig::from_url_and_config_name("/home/alex/.config/nixpkgs", "flink"));

    let mut r = d.run();

    for i in r.out_queue.take().unwrap() {
        println!("got {:?}", i);
        if i == UpgradeState::RequiresReboot {
            println!("sending reboot command");
            r.in_queue.send(UpgradeCommand::Reboot).unwrap();
        }
    }

    println!("result {:?}", r.result.take().unwrap().await.unwrap());

    Ok(())
}

