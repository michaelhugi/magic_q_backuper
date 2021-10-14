use std::borrow::Cow;
use std::fs::{create_dir, remove_dir};
use std::path::{Path, PathBuf};

use serde::*;

use crate::cmdline::{get_number_input, write_green, write_red};
use crate::config::BackupFolder;

#[derive(Debug, Deserialize)]
pub(crate) struct LocalPc {
    src: String,
    dest: String,
    pub(crate) backup_folders: Vec<BackupFolder>,
}

fn str(path: &PathBuf) -> &str {
    path.as_path().as_os_str().to_str().unwrap_or("There may be special chars in your path")
}


impl LocalPc {
    pub(crate) fn validate(&self) -> bool {
        let main_path = Path::new(&self.src);
        if !main_path.exists() {
            write_red(format!("{} for pc system does not exist", self.src).as_str());
            return false;
        }
        if self.backup_folders.len() == 0 {
            write_red("No backup folders specified for local pc system");
            return false;
        }
        for folder in self.backup_folders.iter() {
            let sub_path = main_path.join(Path::new(&folder.folder));
            if !sub_path.exists() {
                write_red(format!("{} for pc system does not exist", str(&sub_path)).as_str())
            }
        }

        true
    }
    //Returns ture if program can continue
    pub(crate) fn backup(&self) -> bool {
        write_green("Backing up pc system...");
        let main_path = Path::new(&self.src);
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
            if remove_dir(&temp).is_err() {
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

    /*   pub(crate) fn backup_folder(&self, src: &PathBuf, dest: &PathBuf, include_subfolders: bool) -> bool {
           if !src.exists() {
               write_red(format!("{} does not exist! ", str(&src)).as_str());
               return false;
           }

           if src.is_file() {
               return self.backup_file(src, dest);
           }

           if !dest.exists() {
               if create_dir(dest).is_err() {
                   write_red(format!("Could not create {}", str(dest)).as_str());
               }
           }

           let files = std::fs::read_dir(src);
           if files.is_err() {
               return false;
           }
           let files = files.unwrap();
           for file in files {
               if file.is_err() {
                   return false;
               }
               let file = file.unwrap().path();
               if file.is_dir() && include_subfolders {
                   let mut parent_dir = file.clone();
                   parent_dir.pop();
                   let parent_dir = str(&dest);
                   let file_name = str(&file.clone()).replace(parent_dir, "");
                   let dest = dest.join(file_name);
                   if !self.backup_folder(&file.as_path().to_path_buf(), &dest, true) {
                       return false;
                   }
               } else if file.is_file() {
                   let mut parent_dir = file.clone();
                   parent_dir.pop();
                   let parent_dir = str(&src);
                   write_red(format!("partent_dir is {}", parent_dir).as_str());
                   let file_name = str(&file.clone()).replace(parent_dir + env::fil, "");
                   write_red(format!("filename is {}", file_name).as_str());
                   write_red(format!("dest is {}", str(dest)).as_str());
                   let dest = dest.join(file_name);
                   if !self.backup_file(&file.as_path().to_path_buf(), &dest) {
                       return false;
                   }
               }
           }

           true
       }


       pub(crate) fn backup_file(&self, src: &PathBuf, dest: &PathBuf) -> bool {
           if std::fs::copy(src, dest).is_err() {
               write_red(format!("Could not copy file {} to {}", str(src), str(dest)).as_str());
               return false;
           }
           true
       }*/

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
            }
        }
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