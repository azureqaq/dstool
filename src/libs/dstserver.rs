//! DSTSERVER Related Modules

#![allow(unused)]

use anyhow::Result;
use std::{
    fmt::Display,
    fs::read_to_string,
    path::{Path, PathBuf},
};

/// DST version information types,
/// including Release,test,unknow
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

/// DST Version status of the DST,
/// including Outdate, Newest, Unknow
pub enum VersionStatus {
    Outdate,
    Newest,
    Unknow,
}

/// DST version information
pub struct DstVersionInfo {
    vtype: DstVersionType,
    number: Option<u32>,
    status: VersionStatus,
}

impl Default for DstVersionInfo {
    fn default() -> Self {
        Self {
            vtype: DstVersionType::Unknow,
            number: None,
            status: VersionStatus::Unknow,
        }
    }
}

impl Display for DstVersionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.vtype {
            DstVersionType::Release => write!(f, "Release"),
            DstVersionType::Test => write!(f, "Test"),
            _ => write!(f, "Unknow"),
        }?;
        if let Some(n) = self.number {
            write!(f, " {}", n)?;
        }
        match self.status {
            VersionStatus::Outdate => write!(f, " Outdate"),
            VersionStatus::Newest => write!(f, " Newest"),
            _ => write!(f, " Unknow"),
        }
    }
}

/// Status information of DST
pub enum DstServerStatus {
    Installed,
    NoInstalled,
    /// Installed but broken
    /// Vec<PathBuf> is the broken files
    /// check bin_32, bin_64 and version_txt
    Broken(Vec<PathBuf>),
    Running,
    Unknow,
}

impl DstServerStatus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_installed(&self) -> bool {
        matches!(self, Self::Installed)
    }

    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }
}

impl Default for DstServerStatus {
    fn default() -> Self {
        Self::Unknow
    }
}

impl Display for DstServerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Installed => write!(f, "Installed"),
            Self::NoInstalled => write!(f, "NoInstalled"),
            Self::Broken(_) => write!(f, "Broken"),
            Self::Running => write!(f, "Running"),
            _ => write!(f, "Unknow"),
        }
    }
}

pub struct DstServerPath {
    /// where to install
    dir_path: PathBuf,
    bin_32: PathBuf,
    bin_64: PathBuf,
    version_txt: PathBuf,
}

impl DstServerPath {
    pub fn new<P: AsRef<Path>>(dir_path: P) -> Self {
        let dir_path = dir_path.as_ref().to_path_buf();
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

    pub fn dir_path(&self) -> &Path {
        self.dir_path.as_path()
    }

    pub fn bin_32(&self) -> &Path {
        self.bin_32.as_path()
    }

    pub fn bin_64(&self) -> &Path {
        self.bin_64.as_path()
    }

    pub fn version_txt(&self) -> &Path {
        self.version_txt.as_path()
    }

    /// Check if the DST is installed,
    /// check bin_32, bin_64 and version_txt,
    /// return true if installed
    pub fn is_installed(&self) -> bool {
        self.bin_32.exists() && self.bin_64.exists() && self.version_txt.exists()
    }
}

impl Display for DstServerPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DstServerPath: {}", self.dir_path.display())
    }
}

/// DstServer object
pub struct DstServer {
    /// Paths used by DstServer
    path: DstServerPath,
    /// The state of the DstServer
    status: DstServerStatus,
    /// DstServer Version Information
    version_info: DstVersionInfo,
}

impl DstServer {
    pub fn new<P: AsRef<Path>>(dir_path: P) -> Self {
        let path = DstServerPath::new(dir_path);
        let status = DstServerStatus::default();
        let version_info = DstVersionInfo::default();
        Self {
            path,
            status,
            version_info,
        }
    }

    pub fn path(&self) -> &DstServerPath {
        &self.path
    }

    pub fn status(&self) -> &DstServerStatus {
        &self.status
    }

    pub fn version_info(&self) -> &DstVersionInfo {
        &self.version_info
    }

    /// Check integrity, if all required files exist,
    /// change their status to Installed,
    /// if none exist then NoInstalled, otherwise Bronken,
    /// and include the path to the missing file.
    pub fn check_integrity(&mut self) {
        let mut broken_files = Vec::new();
        if !self.path.bin_32.exists() {
            broken_files.push(self.path.bin_32.clone());
        }
        if !self.path.bin_64.exists() {
            broken_files.push(self.path.bin_64.clone());
        }
        if !self.path.version_txt.exists() {
            broken_files.push(self.path.version_txt.clone());
        }
        if broken_files.is_empty() {
            self.status = DstServerStatus::Installed;
        } else if broken_files.len() == 3 {
            self.status = DstServerStatus::NoInstalled;
        } else {
            self.status = DstServerStatus::Broken(broken_files);
        }
    }
}
