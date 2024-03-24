pub mod store;
pub mod flake;
pub mod command;

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, ChildStderr};
use std::{io, fs};
use std::io::{BufRead, BufReader};
use mktemp::Temp;
use crate::errors::*;

use store::StorePath;
use flake::FlakeConfig;


#[derive(Debug)]
pub struct BuildOutput {
	pub path: StorePath,
	pub linkdir: Temp,
}

impl BuildOutput {
	fn read_link_dir(linkdir: &Temp) -> Result<StorePath, BuildError> {
		let mut res_path = linkdir.to_path_buf();
		res_path.push("result");
		Ok(StorePath::new(&res_path)?)
	}

	pub fn from_temp(linkdir: Temp) -> Result<Self, BuildError> {
		Ok(Self {
			path: Self::read_link_dir(&linkdir)?,
			linkdir
		})
	}
}

pub trait Buildable {
	fn build(&self) -> Result<BuildOutput, BuildError>;
	fn dry_build(&self) -> Result<StorePath, BuildError>;
}

pub trait Updateable {
	/// return true if the flake inputs (or channel revision) have changed
	fn update(&self) -> Result<bool, UpdateError>;
}

pub struct Profile {
	base_path: PathBuf,
}

impl Profile {
	pub fn new(p: &Path) -> Self {
		Self { base_path: p.into() }
	}

	pub fn system() -> Self {
		Self::new(Path::new("/nix/var/nix/profiles/system"))
	}

	pub fn get_current(&self) -> Result<StorePath, StorePathError> {
		Ok(self.base_path.as_path().try_into()?)
	}
}
