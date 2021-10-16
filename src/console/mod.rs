use serde::*;

use crate::systems::BackupRelPath;

#[derive(Debug, Deserialize, Clone)]
pub struct Console {
    pub name: String,
    pub ip: String,
    pub backup_rel_paths: Vec<BackupRelPath>,
    pub dest: String,
    pub username: String,
    pub password: String,
}

impl Console {
    pub fn validate(&self) -> bool {
        true
    }
    pub fn backup(&self) -> bool { return false; }
}