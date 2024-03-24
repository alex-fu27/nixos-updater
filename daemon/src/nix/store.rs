use crate::errors::*;
use std::path::{Path, PathBuf};
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq, serde_with::DeserializeFromStr)]
pub struct StorePath(PathBuf);

impl StorePath {
	pub fn new(p: &Path) -> Result<Self, StorePathError> {
		let can = match fs::canonicalize(p) {
			Ok(x) => x,
			Err(_) => PathBuf::from(p),
		};
		if ! can.starts_with("/nix/store/") {
			let s = match can.to_str() {
				Some(s) => s,
				None => "unprintable path",
			};
			Err(StorePathError::NotInStore(s.to_string()))
		} else {
			Ok(Self(can))
		}
	}

	pub fn subpath(&self, s: &str) -> PathBuf {
		let mut pb = self.0.clone();
		pb.push(s);
		pb
	}
}

impl std::fmt::Display for StorePath {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.display().fmt(f)
	}
}

impl TryFrom<&Path> for StorePath {
	type Error = StorePathError;

	fn try_from(p: &Path) -> Result<Self, Self::Error> {
		Self::new(p.into())
	}
}

impl TryFrom<PathBuf> for StorePath {
	type Error = StorePathError;

	fn try_from(p: PathBuf) -> Result<Self, Self::Error> {
		Self::new(&p)
	}
}

impl From<&StorePath> for PathBuf {
	fn from(p: &StorePath) -> PathBuf {
		p.0.clone()
	}
}

impl FromStr for StorePath {
	type Err = StorePathError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self::new(&PathBuf::from(s))
	}
}

impl StorePath {
	pub fn as_path(&self) -> &Path {
		self.0.as_path()
	}
}


