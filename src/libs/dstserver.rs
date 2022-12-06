#![allow(unused)]

//! DSTSERVER Related Modules

use anyhow::Result;
use std::{
    fmt::Display,
    fs::read_to_string,
    path::{Path, PathBuf},
};

pub enum DstVersionType {
    Release,
    Test,
    /// Unknow yet
    Unknow,
}

impl Display for DstVersionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Release => write!(f, "Release"),
            Self::Test => write!(f, "Test"),
            _ => write!(f, "Unknow"),
        }
    }
}

pub enum VersionStatus {
    Outdate,
    Newest,
    Unknow,
}

pub struct DstVersionInfo {
    vtype: DstVersionType,
    number: Option<u32>,
}

pub enum DstStatus {
    Installed,
    NoInstalled,
    Broken(Vec<PathBuf>),
    Unknow,
}

impl DstStatus {
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Installed)
    }
}

pub struct DstServerPath {
    /// where to install
    dir_path: PathBuf,
    bin_32: PathBuf,
    bin_64: PathBuf,
    version_txt: PathBuf,
}

pub struct DstServer {
    path: DstServerPath,
    status: DstStatus,
}
