use crate::errors::*;
use super::{Buildable, Updateable};
use super::*;
use super::command::*;

pub struct FlakeConfig {
	pub url: String,
	pub attribute: String,
}

impl FlakeConfig {
	pub fn new(url: &str, attr: &str) -> Self {
		Self {
			url: url.to_string(),
			attribute: attr.to_string()
		}
	}

	pub fn from_url_and_config_name(url: &str, config_name: &str) -> Self {
		Self {
			url: url.to_string(),
			attribute: format!("nixosConfigurations.\"{config_name}\".config.system.build.toplevel"),
		}
	}

	pub fn get_installable(&self) -> String {
		format!("{}#{}", &self.url, &self.attribute)
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

		let mut bnd = child.stdout.take().unwrap();
		let lines = read_to_lines(&mut bnd);
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


