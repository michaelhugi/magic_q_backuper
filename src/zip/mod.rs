extern crate zip;

use std::ffi::OsStr;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::io::Write;
use std::path::{Path, PathBuf};

use zip::write::FileOptions;

use crate::error::Error;
use crate::systems::BackupRelPath;
use crate::tui::TUI;

use self::zip::{CompressionMethod, ZipWriter};

//Zips exactly one file from a src to a zip file while copying
fn zip_one_file_entry(tui: &mut TUI, file: PathBuf, zip: &mut ZipWriter<File>, src_root: &str, options: FileOptions, buffer: &mut Vec<u8>) -> Result<(), Error> {
    tui.update_current_task(format!("Zipping {}", file.display()));
    let relative_name = file.strip_prefix(src_root)?.as_os_str().to_str();
    match relative_name {
        None => Err(Error::new_s("Unexpected error in path calculations")),
        Some(relative_name) => {
            zip.start_file(relative_name, options)?;
            let mut f = File::open(file)?;

            f.read_to_end(buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
            Ok(())
        }
    }
}

//Adds a path to a zip file without content
fn add_path_to_zip(tui: &mut TUI, file: &Path, zip: &mut ZipWriter<File>, src_root: &str, options: FileOptions) -> Result<(), Error> {
    tui.update_current_task(format!("Adding path {} to zip", file.display()));
    let relative_name = file.strip_prefix(src_root)?.as_os_str().to_str();
    match relative_name {
        None => Err(Error::new_s("Unexpected error in path calculations")),
        Some(relative_name) => {
            zip.add_directory(relative_name, options)?;
            Ok(())
        }
    }
}

//Copies a set of user specified paths/files with specified rules about skipping some files or ignoring subdirs in a zip while compressing
pub fn copy_to_zip<S: AsRef<str>>(tui: &mut TUI, src_root_absolute: S, dirs: Vec<BackupRelPath>, dest_zip: &Path) -> Result<(), Error> {
    if dest_zip.exists() {
        return Err(Error::new_s(format!("{} already exists!", dest_zip.display())));
    }
    if dest_zip.extension().and_then(OsStr::to_str).unwrap_or("?") != "zip" {
        return Err(Error::new_s(format!("{} is not a zip file!", dest_zip.display())));
    }
    let src_root = Path::new(src_root_absolute.as_ref());
    if !src_root.exists() {
        return Err(Error::new_s(format!("{} does not exist", src_root_absolute.as_ref())));
    }
    let dest_parent = dest_zip.parent();
    if dest_parent.is_none() {
        return Err(Error::new_s(format!("{} is an invalid path", dest_zip.display())));
    }
    let dest_parent = dest_parent.unwrap();
    if !dest_parent.exists() {
        create_dir_all(dest_parent)?
    }
    let zip_file = File::create(dest_zip)?;
    let mut zip = zip::ZipWriter::new(zip_file);

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();

    for user_specified_dir_to_run in dirs.iter() {
        let mut skip_files = Vec::new();
        match &user_specified_dir_to_run.excluded_files {
            None => {}
            Some(skip) => {
                for skip in skip.iter() {
                    skip_files.push(SkipFile::new(skip))
                }
            }
        }

        let user_specified_dir_to_run_path = src_root.join(&user_specified_dir_to_run.rel_path);
        if !user_specified_dir_to_run_path.exists() {
            return Err(Error::new_s(format!("{} does not exist", user_specified_dir_to_run_path.display())));
        }
        if user_specified_dir_to_run_path.is_file() {
            zip_one_file_entry(tui, user_specified_dir_to_run_path, &mut zip, &src_root_absolute.as_ref(), options, &mut buffer)?;
        } else {
            let mut dir_tree_to_run = vec![src_root.join(&user_specified_dir_to_run_path)];
            while let Some(dir_in_to_run_tree) = dir_tree_to_run.pop() {
                //Can't be file at this point
                for file_or_subdir in std::fs::read_dir(dir_in_to_run_tree)?.into_iter() {
                    let file_or_subdir = file_or_subdir?.path();
                    if file_or_subdir.is_file() {
                        if file_is_excluded(&file_or_subdir, &skip_files) {
                            tui.update_current_task(format!("Skipping file {} because it's excluded", file_or_subdir.display()));
                        } else {
                            zip_one_file_entry(tui, file_or_subdir, &mut zip, &src_root_absolute.as_ref(), options, &mut buffer)?;
                        }
                    } else if user_specified_dir_to_run.include_subfolders {
                        add_path_to_zip(tui, &file_or_subdir, &mut zip, &src_root_absolute.as_ref(), options)?;
                        dir_tree_to_run.push(file_or_subdir);
                    }
                }
            }
        }
    }
    tui.update_current_task("All entries zipped...");
    zip.finish()?;
    Ok(())
}

//Says if a file should be excluded due to rules the user said
fn file_is_excluded(file: &Path, skip_files: &[SkipFile]) -> bool {
    for skip_file in skip_files.iter() {
        if skip_file.skip(file) {
            return true;
        }
    }
    false
}

//Holds a rule what files to skip (either *.extension for all files with this extension or an explicit filename)
struct SkipFile {
    name: String,
    is_extension: bool,
}

impl SkipFile {
    fn new(value: &str) -> Self {
        let mut is_extension = false;
        let name = if value.starts_with("*.") {
            is_extension = true;
            value.replace("*.", "")
        } else {
            value.to_string()
        };
        Self {
            name,
            is_extension,
        }
    }
    fn skip(&self, path: &Path) -> bool {
        if self.is_extension {
            self.name == path.extension().and_then(OsStr::to_str).unwrap_or("?")
        } else {
            self.name == path.file_name().and_then(OsStr::to_str).unwrap_or("?")
        }
    }
}
