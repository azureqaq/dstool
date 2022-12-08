//! DSTSERVER Related Modules
//!
//! Don't Starve Together Server Dedicated Server

#![allow(unused)]

use anyhow::Result;
use std::path::{Path, PathBuf};

/// DST Server
///
/// Only the official version is supported,
/// not the beta version
pub struct DstServer {
    /// dir path of the server
    dir_path: PathBuf,
    /// bin_32
    bin_32: PathBuf,
    /// bin_64
    bin_64: PathBuf,
    /// version.txt
    version_txt: PathBuf,
}

impl DstServer {
    /// Create a new DST Server
    pub fn new(dir_path: PathBuf) -> Self {
        let bin_32 = dir_path
            .join("bin")
            .join("dontstarve_dedicated_server_nullrenderer.exe");
        let bin_64 = dir_path
            .join("bin_64")
            .join("dontstarve_dedicated_server_nullrenderer_x64.exe");
        let version_txt = dir_path.join("version.txt");
        Self {
            dir_path,
            bin_32,
            bin_64,
            version_txt,
        }
    }

    /// Get the dir path of the server
    pub fn dir_path(&self) -> &Path {
        self.dir_path.as_path()
    }

    /// Get the bin_32 path of the server
    pub fn bin_32(&self) -> &Path {
        self.bin_32.as_path()
    }

    /// Get the bin_64 path of the server
    pub fn bin_64(&self) -> &Path {
        self.bin_64.as_path()
    }

    /// Get the version.txt path of the server
    pub fn version_txt(&self) -> &Path {
        self.version_txt.as_path()
    }

    /// Get the version number of the server
    pub fn version(&self) -> Result<u32> {
        let content = std::fs::read_to_string(&self.version_txt)?;
        let version = content.trim().parse::<u32>()?;
        Ok(version)
    }
}
