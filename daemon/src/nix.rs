use std::path::{Path, PathBuf};
use mktemp::Temp;

#[derive(Debug)]
pub struct BuildOutput {
    path: StorePath,
    linkdir: Temp,
}

impl BuildOutput {
    fn read_link_dir(linkdir: &Temp) -> Result<StorePath, StorePathError> {
        let mut res_path = linkdir.to_path_buf();
        res_path.push("result");
        let res_path = res_path.read_link().map_err(|e| StorePathError::NotInStore(format!("bad out link: {}", e)))?;
        Ok(StorePath::new(res_path)?)
    }

    pub fn from_temp(linkdir: Temp) -> Result<Self, StorePathError> {
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

