use std::fs::create_dir;
use std::path::{Path, PathBuf};

use serde::*;

use crate::copy::collect_copy_folders;
use crate::systems::BackupRelPath;
use crate::tui::TUI;

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
    pub fn validate(&self, tui: &mut TUI) -> bool {
        let main_path = Path::new(&self.src);
        if !main_path.exists() {
            tui.write_errorln(format!("{} for pc system does not exist", self.src));
            return false;
        }
        if self.backup_rel_paths.len() == 0 {
            tui.write_errorln("No backup folders specified for local pc system");
            return false;
        }
        for folder in self.backup_rel_paths.iter() {
            let sub_path = main_path.join(Path::new(&folder.rel_path));
            if !sub_path.exists() {
                tui.write_errorln(format!("{} for pc system does not exist", str(&sub_path)))
            }
        }
        true
    }
    //Returns ture if program can continue
    pub fn backup(self, tui: &mut TUI) -> Result<(), std::io::Error> {
        tui.write_title(format!("Backing up {}", self.name));

        let temp = Path::new(&self.dest).join("temp");
        if temp.exists() {
            std::fs::remove_dir_all(&temp)?;
        }
        create_dir(&temp)?;

        tui.writeln("Calculating folders. Please wait...");

        let recursive_folders = collect_copy_folders(&self.backup_rel_paths, Path::new(&self.src).to_path_buf(), Path::new(&temp).to_path_buf())?;

        let mut total_size = 0f64;
        let mut size_done = 0f64;
        let mut last_percentage = 0usize;

        for folder in recursive_folders.iter() {
            total_size += folder.size_without_sub_folders;
        }
        if total_size == 0f64 {
            total_size = 1f64;
        }

        for folder in recursive_folders.iter() {
            let out = folder.backup_all_files(&size_done, &total_size, &last_percentage, tui)?;
            size_done = out.0;
            last_percentage = out.1;
        }
        Ok(())
    }
}

