use std::fs::create_dir_all;
use std::path::Path;

use serde::*;

use crate::error::Error;
use crate::systems::BackupRelPath;
use crate::tui::TUI;
use crate::zip::copy_to_zip;
use crate::zip_name::get_zip_path;

#[derive(Debug, Deserialize, Clone)]
pub struct LocalInstallation {
    pub name: String,
    src: String,
    dest: String,
    pub backup_rel_paths: Vec<BackupRelPath>,
}

impl LocalInstallation {
    //Validates if the specified path and its specified paths exist. Otherwise it returns an error with information to show to the user
    pub fn validate(&self) -> Result<(), Error> {
        let main_path = Path::new(&self.src);
        if !main_path.exists() {
            return Err(Error::new_s(format!("{} for {} system does not exist", self.src, self.name)));
        }
        if self.backup_rel_paths.is_empty() {
            return Err(Error::new_s(format!("No backup folders specified for {} system", self.name)));
        }
        for folder in self.backup_rel_paths.iter() {
            let sub_path = main_path.join(Path::new(&folder.rel_path));
            if !sub_path.exists() {
                return Err(Error::new_s(format!("{} for {} does not exist", sub_path.display(), self.name)));
            }
        }
        Ok(())
    }
    //Returns ture if program can continue
    pub fn backup(self, tui: &mut TUI) -> Result<String, Error> {
        tui.write_title(format!("Backing up {}", self.name));

        let dest = Path::new(&self.dest);
        if !dest.exists() {
            create_dir_all(dest)?;
        }
        let dest_zip = get_zip_path(&self.name, dest);
        tui.writeln(format!("Creating {}\n", dest_zip.display()));
        copy_to_zip(tui, self.src, self.backup_rel_paths, &dest_zip)?;
        Ok(format!("\nCreated backup file for {}:\n{}\n\n", self.name, dest_zip.display()))
    }
}

