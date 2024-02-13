use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use mktemp::Temp;

#[derive(Debug)]
pub enum BuildError {
    StorePathError(StorePathError),
    IOError(io::Error),
}

impl From<io::Error> for BuildError {
    fn from(e: io::Error) -> Self {
        Self::IOError(e)
    }
}

impl From<StorePathError> for BuildError {
    fn from(e: StorePathError) -> Self {
        Self::StorePathError(e)
    }
}

#[derive(Debug)]
pub enum UpdateError {
    IOError(io::Error),
}

impl From<io::Error> for BuildError {
    fn from(e: io::Error) -> Self {
        Self::IOError(e)
    }
}


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

#[derive(Debug)]
pub enum StorePathError {
    NotInStore(String),
}

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
    fn build() -> Result<BuildOutput, BuildError>;
}

pub trait Updateable {
    fn Update() -> Result<(), UpdateError>;
}

pub struct Flake {
    url: String,
}

impl Flake {
    pub fn from_url(url: &str) -> Self {
        Self { url: url.to_string() }
    }
}

impl Buildable for Flake {
    fn build(&self) -> Result<BuildOutput, BuildError> {
        let wd = Temp::new_dir()?;
        let mut child = Command::new("nix")
            .stdin(Stdio::null())
            .stderr(Stdio::piped())
            .stdout(Stdio::null())
            .current_dir(&wd.as_path())
            .args(["--extra-experimental-features", "nix-command flakes",
                "--log-format", "internal-json", "-vv",
                "build", &self.url])
            .spawn()?;
        let stderr = read_to_lines(&mut child.stderr);

        for line in stderr.flatten() {
            log::debug!("{}", line);
        }

        child.wait()?;

        Ok(BuildOutput::from_temp(wd)?)
    }
}
