use std::fs::{create_dir, create_dir_all};
use std::path::{Path, PathBuf};

use crate::systems::BackupRelPath;
use crate::tui::TUI;

pub struct CopyQueueFolder<SRC: AsRef<Path>, DEST: AsRef<Path>> {
    pub src_path: SRC,
    pub dest_path: DEST,
    pub size_without_sub_folders: f64,
}

impl<SRC: AsRef<Path>, DEST: AsRef<Path>> CopyQueueFolder<SRC, DEST> {
    pub fn readable_src_path(&self) -> &str {
        self.src_path.as_ref().as_os_str().to_str().unwrap_or("There may be special chars in your path")
    }

    pub fn backup_all_files(&self, size_done: &f64, total_size: &f64, last_percentage: &usize, tui: &mut TUI) -> Result<(f64, usize), std::io::Error> {
        let mut size_done = size_done.clone();
        let mut last_percentage = last_percentage.clone();
        let task = format!("Backing up {}", &self.readable_src_path());
        let src = self.src_path.as_ref();
        let dest = self.dest_path.as_ref();
        if src.is_dir() {
            if !dest.exists() {
                create_dir(dest)?;
            }
            for entry in std::fs::read_dir(src)? {
                let entry = entry?.path();
                if entry.is_file() {
                    let file_name = entry.file_name().unwrap();
                    let dest = dest.join(file_name);
                    std::fs::copy(&entry, &dest)?;
                    size_done += entry.metadata()?.len() as f64;
                    last_percentage = tui.write_progress(&size_done, total_size, &last_percentage, task.as_str());
                }
            }
        } else {
            let dest_parent = dest.parent().unwrap();
            if !dest_parent.exists() {
                create_dir_all(dest_parent)?;
            }
            std::fs::copy(&src, &dest)?;
            size_done += src.metadata()?.len() as f64;
            last_percentage = tui.write_progress(&size_done, total_size, &last_percentage, task.as_str());
        }
        Ok((size_done, last_percentage))
    }
}

pub fn new_copy_queue_folder<SRC: AsRef<Path>, DEST: AsRef<Path>>(src_path: SRC, dest_path: DEST) -> Result<CopyQueueFolder<SRC, DEST>, std::io::Error> {
    let mut size_without_sub_folders = 0f64;
    if src_path.as_ref().is_file() {
        size_without_sub_folders = src_path.as_ref().metadata()?.len() as f64;
    } else {
        for entry in std::fs::read_dir(&src_path)? {
            let file_path_in_src = entry?.path();
            if file_path_in_src.is_file() {
                size_without_sub_folders += file_path_in_src.metadata()?.len() as f64;
            }
        }
    }
    Ok(CopyQueueFolder {
        src_path,
        dest_path,
        size_without_sub_folders,
    })
}


pub fn collect_copy_folders(requested_copy_folders: &Vec<BackupRelPath>, src_root: PathBuf, dest_root: PathBuf) -> Result<Vec<CopyQueueFolder<PathBuf, PathBuf>>, std::io::Error> {
    let mut out_folders = Vec::new();
    let mut requested_copy_folders = requested_copy_folders.clone();
    let src_root_size = PathBuf::from(&src_root).components().count();
    while let Some(requested_copy_folder) = requested_copy_folders.pop() {
        let mut src_path = src_root.clone();
        src_path.push(&requested_copy_folder.rel_path);
        let mut dest_path = dest_root.clone();
        dest_path.push(&requested_copy_folder.rel_path);

        if src_path.is_file() {
            out_folders.push(new_copy_queue_folder(src_path, dest_path)?)
        } else {
            let mut copy_folders_stack = Vec::new();
            copy_folders_stack.push(src_path);
            while let Some(src_path) = copy_folders_stack.pop() {
                let rel_path: PathBuf = src_path.components().skip(src_root_size).collect();
                let dest_path = if rel_path.components().count() == 0 {
                    dest_root.clone()
                } else {
                    dest_root.clone().join(&rel_path)
                };
                out_folders.push(new_copy_queue_folder(src_path.clone(), dest_path)?);
                if requested_copy_folder.include_subfolders {
                    for entry in std::fs::read_dir(src_path)? {
                        let entry = entry?.path();
                        if entry.is_dir() {
                            copy_folders_stack.push(entry)
                        }
                    }
                }
            }
        }
    }

    Ok(out_folders)
}
