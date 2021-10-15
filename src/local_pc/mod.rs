use std::fs::create_dir;
use std::path::{Path, PathBuf};

use serde::*;

use crate::cmdline::{Cmdline, SEPARATOR_LINE};
use crate::config::BackupRelPath;
use crate::copy::collect_copy_folders;

#[derive(Debug, Deserialize)]
pub(crate) struct LocalPc {
    src: String,
    dest: String,
    pub(crate) backup_rel_paths: Vec<BackupRelPath>,
}

fn str(path: &PathBuf) -> &str {
    path.as_path().as_os_str().to_str().unwrap_or("There may be special chars in your path")
}


impl LocalPc {
    pub(crate) fn validate(&self, cmd: &mut Cmdline) -> bool {
        let main_path = Path::new(&self.src);
        if !main_path.exists() {
            cmd.write_red(format!("{} for pc system does not exist", self.src).as_str());
            return false;
        }
        if self.backup_rel_paths.len() == 0 {
            cmd.write_red("No backup folders specified for local pc system");
            return false;
        }
        for folder in self.backup_rel_paths.iter() {
            let sub_path = main_path.join(Path::new(&folder.rel_path));
            if !sub_path.exists() {
                cmd.write_red(format!("{} for pc system does not exist", str(&sub_path)).as_str())
            }
        }

        true
    }
    //Returns ture if program can continue
    pub(crate) fn backup(self, cmd: &mut Cmdline) -> Result<(), std::io::Error> {
        cmd.write_green(SEPARATOR_LINE);
        cmd.write_green("Backing up pc system");
        cmd.write_green(SEPARATOR_LINE);

        let temp = Path::new(&self.dest).join("temp");
        if temp.exists() {
            std::fs::remove_dir_all(&temp)?;
        }
        create_dir(&temp)?;

        cmd.write_green("Calculating folders. Please wait...");

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
            let out = folder.backup_all_files(&size_done, &total_size, &last_percentage, cmd)?;
            size_done = out.0;
            last_percentage = out.1;
        }
        Ok(())
    }
}

