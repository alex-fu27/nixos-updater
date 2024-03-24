pub mod store;
pub mod flake;

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

fn read_to_lines<T: io::Read>(o: &mut T) -> io::Lines<io::BufReader<&mut T>> {
	BufReader::new(o).lines()
}

fn nix_command() -> Command {
	let mut cmd = Command::new("nix");
	cmd.stdin(Stdio::null())
		.stderr(Stdio::piped())
		.stdout(Stdio::null())
		.args(["--extra-experimental-features", "nix-command flakes",
				"-vv",]);
	cmd
}

fn output_stderr_as_debug(stderr: &mut ChildStderr) {
	let stderr = read_to_lines(stderr);

	for line in stderr.flatten() {
		log::debug!("{}", line);
	}
}

impl Buildable for FlakeConfig {
	fn build(&self) -> Result<BuildOutput, BuildError> {
		let wd = Temp::new_dir()?;
		let installable = self.get_installable();
		let mut child = nix_command()
			.current_dir(&wd.as_path())
			.args(["build", &installable])
			.spawn()?;
		
		output_stderr_as_debug(&mut child.stderr.take().unwrap());
		if ! child.wait()?.success() {
			Err(BuildError::NixCommandFailed)?;
		}

		Ok(BuildOutput::from_temp(wd)?)
	}

	fn dry_build(&self) -> Result<StorePath, BuildError> {
		let wd = Temp::new_dir()?;
		let installable = self.get_installable();
		let mut child = nix_command()
			.current_dir(&wd.as_path())
			.args(["build", "--json", "--dry-run", &installable])
			.spawn()?;
		
		output_stderr_as_debug(&mut child.stderr.take().unwrap());
		if ! child.wait()?.success() {
			Err(BuildError::NixCommandFailed)?;
		}

		  let lines = read_to_lines(&mut child.stdout.take().unwrap());
		  let last = lines.flatten().reduce(|_, a| a).expect("nix build --dry-run has not produced an output list");


		  todo!()
	}
}

impl Updateable for FlakeConfig {
	fn update(&self) -> Result<bool, UpdateError> {
		let mut child = nix_command()
			.args(["flake", "update", &self.url])
			.spawn()?;
		
		let mut bind = child.stderr.take().unwrap();
		let stderr = read_to_lines(&mut bind);
		let mut has_update = false;
		for line in stderr.flatten() {
			log::debug!("{}", line);
			if line.contains("updating lock file") {
				has_update = true;
			}
		}
		if ! child.wait()?.success() {
			Err(UpdateError::NixCommandFailed)?;
		}

		Ok(has_update)
	}
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

#[cfg(tests)]
mod tests {
	#[test]
	fn testtest() {
		assert_eq!(1, 0);
	}
}
