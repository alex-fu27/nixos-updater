use thiserror::Error;
use std::io;

#[derive(Debug, Error)]
pub enum StorePathError {
    #[error("path {} is not in nix store", .0)]
    NotInStore(String),
}

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("build error: {}", .0)]
    StorePathError(#[from] StorePathError),
    #[error("build error: {}", .0)]
    IOError(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("update failed: {}", .0)]
    IOError(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum DaemonError {
    #[error("update error: {}", .0)]
    StorePathError(#[from] StorePathError),
    #[error("update error: {}", .0)]
    IOError(#[from] io::Error),
}

