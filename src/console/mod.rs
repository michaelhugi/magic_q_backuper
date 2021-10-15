use serde::*;

use crate::config::BackupRelPath;

#[derive(Debug, Deserialize)]
pub(crate) struct Console {
    pub(crate) name: String,
    pub(crate) ip: String,
    pub(crate) backup_rel_paths: Vec<BackupRelPath>,
    pub(crate) dest: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Console {
    pub(crate) fn validate(&self) -> bool {
        true
    }
    pub(crate) fn backup(&self) -> bool { return false; }
}