//! DSTCLUSTER
//!
//! Don't Starve Together Cluster

#![allow(unused)]

use std::path::{PathBuf, Path};
use anyhow::Result;
use crate::dstserver::DstServer;

/// DST Cluster Config
pub struct DstClusterConfig {
    /// admin.txt list
    admin_list: Vec<String>,
    /// whitelist.txt list
    whitelist_list: Vec<String>,
    /// blocklist.txt list
    blocklist_list: Vec<String>,
    /// mod id list
    mod_list: Vec<String>,
}

impl DstClusterConfig {
    /// Create a new DST Cluster Config
    pub fn new() -> Self {
        Self {
            admin_list: Vec::new(),
            whitelist_list: Vec::new(),
            blocklist_list: Vec::new(),
            mod_list: Vec::new(),
        }
    }

    /// new form cluster dir path
    pub fn from_cluster_dir_path<P: AsRef<Path>>(cluster_dir_path: P) -> Result<Self> {
        let cluster_dir_path = cluster_dir_path.as_ref();
        let mut admin_list = Vec::new();
        let mut whitelist_list = Vec::new();
        let mut blocklist_list = Vec::new();
        let mut mod_list = Vec::new();

        let admin_txt_path = cluster_dir_path.join("admin.txt");
        let whitelist_txt_path = cluster_dir_path.join("whitelist.txt");
        let blocklist_txt_path = cluster_dir_path.join("blocklist.txt");
        let modoverrides_path = cluster_dir_path.join("modoverrides.lua");

        if admin_txt_path.exists() {
            let admin_txt = std::fs::read_to_string(admin_txt_path)?;
            admin_list = admin_txt
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        if whitelist_txt_path.exists() {
            let whitelist_txt = std::fs::read_to_string(whitelist_txt_path)?;
            whitelist_list = whitelist_txt
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        if blocklist_txt_path.exists() {
            let blocklist_txt = std::fs::read_to_string(blocklist_txt_path)?;
            blocklist_list = blocklist_txt
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        if modoverrides_path.is_file() {
            let modoverrides = std::fs::read_to_string(modoverrides_path)?;
            let modoverrides = modoverrides
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>();
            let mut mod_id = String::new();
            for line in modoverrides {
                if line.starts_with("Workshop") {
                    mod_id = line
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .trim()
                        .trim_matches('"')
                        .to_string();
                } else if line.starts_with("[") {
                    if !mod_id.is_empty() {
                        mod_list.push(mod_id);
                        mod_id = String::new();
                    }
                }
            }
        }

        Ok(
            Self {
                admin_list,
                whitelist_list,
                blocklist_list,
                mod_list,
            }
        )
    }

    /// Get the admin.txt list
    pub fn admin_list(&self) -> &[String] {
        self.admin_list.as_slice()
    }

    /// Get the whitelist.txt list
    pub fn whitelist_list(&self) -> &[String] {
        self.whitelist_list.as_slice()
    }

    /// Get the blocklist.txt list
    pub fn blocklist_list(&self) -> &[String] {
        self.blocklist_list.as_slice()
    }

    /// Get the mod id list
    pub fn mod_list(&self) -> &[String] {
        self.mod_list.as_slice()
    }

    /// Add a admin.txt
    pub fn add_admin(&mut self, admin: String) {
        self.admin_list.push(admin);
    }

    /// Add a whitelist.txt
    pub fn add_whitelist(&mut self, whitelist: String) {
        self.whitelist_list.push(whitelist);
    }

    /// Add a blocklist.txt
    pub fn add_blocklist(&mut self, blocklist: String) {
        self.blocklist_list.push(blocklist);
    }

    /// Add a mod id
    pub fn add_mod(&mut self, mod_id: String) {
        self.mod_list.push(mod_id);
    }
}


/// DST Cluster
pub struct DstCluster {
    /// DST Server, the main server
    main_server: DstServer,

    /// DST Server, the cave server,
    /// if need cave, this is Some(DstServer),
    /// otherwise, this is None
    cave_server: Option<DstServer>,

    /// dir path of the cluster
    dir_path: PathBuf,

    /// config
    config: DstClusterConfig,
}
