use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, ChildStderr};
use std::io;
use std::io::{BufRead, BufReader};
use mktemp::Temp;
use crate::errors::*;

#[derive(Debug)]
pub struct BuildOutput {
    path: StorePath,
    linkdir: Temp,
}

impl BuildOutput {
    fn read_link_dir(linkdir: &Temp) -> Result<StorePath, BuildError> {
        let mut res_path = linkdir.to_path_buf();
        res_path.push("result");
        let res_path = res_path.read_link()?;
        Ok(StorePath::new(res_path)?)
    }

    pub fn from_temp(linkdir: Temp) -> Result<Self, BuildError> {
        Ok(Self {
            path: Self::read_link_dir(&linkdir)?,
            linkdir
        })
    }
}

#[derive(Debug)]
pub struct StorePath(PathBuf);

impl StorePath {
    pub fn new(p: PathBuf) -> Result<Self, StorePathError> {
        if ! p.starts_with("/nix/store/") {
            let s = match p.to_str() {
                Some(s) => s,
                None => "unprintable path",
            };
            Err(StorePathError::NotInStore(s.to_string()))
        } else {
            Ok(Self(p.into()))
        }
    }
}

impl TryFrom<&Path> for StorePath {
    type Error = StorePathError;

    fn try_from(p: &Path) -> Result<Self, Self::Error> {
        Self::new(p.into())
    }
}

impl StorePath {
    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }
}

pub trait Buildable {
    fn build(&self) -> Result<BuildOutput, BuildError>;
}

pub trait Updateable {
    /// return true if the flake inputs (or channel revision) have changed
    fn update(&self) -> Result<bool, UpdateError>;
}

pub struct Flake {
    url: String,
}

impl Flake {
    pub fn from_url(url: &str) -> Self {
        Self { url: url.to_string() }
    }
}

fn read_to_lines<T: io::Read>(o: &mut T) -> io::Lines<io::BufReader<&mut T>> {
    BufReader::new(o).lines()
}

fn nix_command() -> Command {
    let mut cmd = Command::new("nix");
    cmd.stdin(Stdio::null())
        .stderr(Stdio::piped())
        .stdout(Stdio::null())
        .args(["--extra-experimental-features", "nix-command flakes",
                "--log-format", "internal-json", "-vv",]);
    cmd
}

fn output_stderr_as_debug(stderr: &mut ChildStderr) {
    let stderr = read_to_lines(stderr);

    for line in stderr.flatten() {
        log::debug!("{}", line);
    }
}

impl Buildable for Flake {
    fn build(&self) -> Result<BuildOutput, BuildError> {
        let wd = Temp::new_dir()?;
        let mut child = nix_command()
            .current_dir(&wd.as_path())
            .args(["build", &self.url])
            .spawn()?;
        
        output_stderr_as_debug(&mut child.stderr.take().unwrap());

        child.wait()?;

        Ok(BuildOutput::from_temp(wd)?)
    }
}

impl Updateable for Flake {
    fn update(&self) -> Result<bool, UpdateError> {
        let mut child = nix_command()
            .args(["flake", "update", &self.url])
            .spawn()?;
        
        let stderr = read_to_lines(child.stderr.take().unwrap);
        let mut has_update = false;
        for line in stderr.flatten() {
            log::debug!("{}", line);
            if line.contains("updating lock file") {
                has_update = true;
            }
        }
        child.wait()?;

        Ok(has_update)
    }
}
