use std::path::{Path, PathBuf};

use serde::*;

use crate::error::{Error, new_error_s};
use crate::systems::BackupRelPath;
use crate::tui::TUI;
use crate::zip::copy_to_zip;

#[derive(Debug, Deserialize, Clone)]
pub struct LocalInstallation {
    pub name: String,
    src: String,
    dest: String,
    pub backup_rel_paths: Vec<BackupRelPath>,
}

fn str(path: &PathBuf) -> &str {
    path.as_path().as_os_str().to_str().unwrap_or("There may be special chars in your path")
}


impl LocalInstallation {
    //Validates if the specified path and its specified paths exist. Otherwise it returns an error with information to show to the user
    pub fn validate(&self) -> Result<(), Error> {
        let main_path = Path::new(&self.src);
        if !main_path.exists() {
            return Err(new_error_s(format!("{} for {} system does not exist", self.src, self.name)));
        }
        if self.backup_rel_paths.len() == 0 {
            return Err(new_error_s(format!("No backup folders specified for {} system", self.name)));
        }
        for folder in self.backup_rel_paths.iter() {
            let sub_path = main_path.join(Path::new(&folder.rel_path));
            if !sub_path.exists() {
                return Err(new_error_s(format!("{} for {} does not exist", str(&sub_path), self.name)));
            }
        }
        Ok(())
    }
    //Returns ture if program can continue
    pub fn backup(self, tui: &mut TUI) -> Result<String, Error> {
        tui.write_title(format!("Backing up {}", self.name));

        let dest_zip = "Temp.zip";
        copy_to_zip(tui, self.src, self.backup_rel_paths, dest_zip)?;
        Ok("Judihui".to_string())
    }
}

