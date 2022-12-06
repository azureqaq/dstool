//! DSTSERVER Related Modules

use anyhow::Result;
use std::{fs::read_to_string, path::Path};

pub enum DstVersion {
    Release(u32),
    Test(u32),
    /// Unknow yet
    Unknow(u32),
}

pub enum DstStatus<'a > {
    Installed,
    NoInstalled,
    Incomplete(Vec<&'a Path>),
}

pub trait DstServer {
    /// the version info of `DstServer`
    fn dstversion(&self) -> Result<DstVersion> {
        let content = read_to_string(self.version_txt_path())?;
        let num: u32 = content.trim().parse()?;
        Ok(DstVersion::Unknow(num))
    }

    /// the bin_32 path
    fn bin_32(&self) -> &Path;

    /// the bin_64 path
    fn bin_64(&self) -> &Path;

    /// version.txt file_path
    fn version_txt_path(&self) -> &Path;

    /// folder (where to install dst)
    fn folder(&self) -> &Path;

    /// status
    fn status(&self) -> DstStatus;
}
