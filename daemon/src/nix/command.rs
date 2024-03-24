use crate::errors::*;
use super::store::StorePath;
use std::process::{Command, Stdio, ChildStderr};
use std::io::{Read, BufRead, BufReader, Lines};
use std::collections::HashMap;
use serde_with::{serde_as, DisplayFromStr};

pub fn nix_command() -> Command {
	let mut cmd = Command::new("nix");
	cmd.stdin(Stdio::null())
		.stderr(Stdio::piped())
		.stdout(Stdio::piped())
		.args(["--extra-experimental-features", "nix-command flakes",
				"-vv",]);
	cmd
}

pub fn output_stderr_as_debug(stderr: &mut ChildStderr) {
	let stderr = read_to_lines(stderr);

	for line in stderr.flatten() {
		log::debug!("{}", line);
	}
}

pub fn read_to_lines<T: Read>(o: &mut T) -> Lines<BufReader<&mut T>> {
	BufReader::new(o).lines()
}

#[serde_as]
#[derive(Debug, serde::Deserialize)]
pub struct DrvResultInfo {
	#[serde(rename="drvPath")]
	pub drv_path: StorePath,
	pub outputs: HashMap<String, StorePath>,
}

#[cfg(test)]
mod tests {
	 use super::*;
	 #[test]
	 fn parse_drv_result_info() {
		  let data = r#"[{"drvPath":"/nix/store/k6qyppd2y8yamyx7vrq3zd9vac5hgc5n-hello-2.12.1.drv","outputs":{"out":"/nix/store/rnxji3jf6fb0nx2v0svdqpj9ml53gyqh-hello-2.12.1"}}]"#;
		  let v: Vec<DrvResultInfo> = serde_json::from_str(data).unwrap();
		  println!("{:?}", v);
	 }
}
