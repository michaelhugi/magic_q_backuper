use std::borrow::Cow;
use std::fmt::Error;
use std::fs::{create_dir, remove_dir};
use std::io::ErrorKind;
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

        let mut recursive_folders = collect_copy_folders(&self.backup_rel_paths, Path::new(&self.src).to_path_buf(), Path::new(&temp).to_path_buf())?;

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

        /*  let main_path = Path::new(&self.src);
          if !main_path.exists() {
              write_red(format!("{} does not exist", &self.src).as_str());
              return self.show_error_message();
          }
          let dest = Path::new(&self.dest);
          if !dest.exists() {
              if create_dir(dest).is_err() {
                  write_red(format!("Could not create folder {}", self.dest).as_ref());
                  return false;
              }
          }
          let temp = dest.join("temp");
          if temp.exists() {
              if std::fs::remove_dir_all(&temp).is_err() {
                  write_red(format!("Could not remove temporary folder {}", str(&temp)).as_ref());
                  return false;
              }
          }
          if create_dir(&temp).is_err() {
              write_red(format!("Could not create temporary folder {}", str(&temp)).as_ref());
              return false;
          }
          let temp = temp.as_path();
          for folder in self.backup_folders.iter() {
              let sub_path = main_path.join(Path::new(&folder.folder));
              if !sub_path.exists() {
                  write_red(format!("{} for pc system does not exist", str(&sub_path)).as_str())
              }
              let src = main_path.join(&folder.folder);
              let dest = temp.join(&folder.folder);
              if copy_folder(src, dest, folder.include_subfolders).is_err() {
                  return false;
              }
          }
          write_green("Pc system backuped");
          true
      }

      fn show_error_message(&self) -> bool {
          write_red("Could not backup local pc system");
          write_red("1) Abort");
          write_red("2) Continue");

          match get_number_input() {
              1 => false,
              2 => true,
              _ => {
                  write_red("Invalid input");
                  self.show_error_message()
              } }*/
    }
}


pub fn copy_folder<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V, include_subfolders: bool) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        println!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if std::fs::metadata(&dest).is_err() {
            println!(" mkdir: {:?}", dest);
            std::fs::create_dir_all(&dest)?;
        }

        for entry in std::fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if include_subfolders {
                    stack.push(path);
                }
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        println!("  copy: {:?} -> {:?}", &path, &dest_path);
                        std::fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}

