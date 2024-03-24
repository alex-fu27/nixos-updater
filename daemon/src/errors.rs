use thiserror::Error;
use std::io;

#[derive(Debug, Error)]
pub enum StorePathError {
	#[error("path {} is not in nix store", .0)]
	NotInStore(String),
	#[error("io error: {}", .0)]
	IOError(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum BuildError {
	#[error("build error: {}", .0)]
	StorePathError(#[from] StorePathError),
	#[error("build error: {}", .0)]
	IOError(#[from] io::Error),
	#[error("nix command failed")]
	NixCommandFailed,
	#[error("flake has no attribute")]
	FlakeHasNoAttr,
	#[error("nix build --json output could not be parsed: {}", .0)]
	ParsingNixBuildJSONFailed(serde_json::Error),
	#[error("nix build --dry-run produced unexepcted output: {}", .0)]
	DryRunProducedUnexpected(String),
}

#[derive(Debug, Error)]
pub enum UpdateError {
	#[error("update failed: {}", .0)]
	IOError(#[from] io::Error),
	#[error("nix command failed")]
	NixCommandFailed,
}

#[derive(Debug, Error)]
pub enum UpgradeError {
	#[error("upgrade process failed: {}", .0)]
	BuildError(#[from] BuildError),
	#[error("upgrade process failed: {}", .0)]
	UpdateError(#[from] UpdateError),
	#[error("upgrade failed: {}", .0)]
	StorePathError(#[from] StorePathError),
	#[error("switch command failed: {:?}", .0)]
	SwitchFailed(Option<io::Error>),
	#[error("reboot failed: {:?}", .0)]
	RebootFailed(String),
	#[error("user cancelled operation")]
	Cancelled,
}

impl UpgradeError {
	pub fn map_switch_io_error(e: io::Error) -> UpgradeError {
		Self::SwitchFailed(Some(e))
	}

	pub fn map_reboot_failed(e: impl std::error::Error) -> UpgradeError {
		Self::RebootFailed(e.to_string())
	}
}

