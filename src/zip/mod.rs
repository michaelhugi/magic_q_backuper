extern crate walkdir;
extern crate zip;

use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::io::Write;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;
use zip::write::FileOptions;

use crate::error::{Error, new_error_s};
use crate::systems::BackupRelPath;
use crate::tui::TUI;

use self::zip::{CompressionMethod, ZipWriter};
use self::zip::read::ZipFile;

fn zip_one_file_entry(tui: &mut TUI, file: PathBuf, zip: &mut ZipWriter<File>, src_root: &str, options: FileOptions, buffer: &mut Vec<u8>) -> Result<(), Error> {
    tui.update_current_task(format!("Zipping {}", file.display()));
    let relative_name = file.strip_prefix(src_root)?.as_os_str().to_str();
    match relative_name {
        None => Err(new_error_s("Unexpected error in path calculations")),
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

fn add_path_to_zip(tui: &mut TUI, file: &PathBuf, zip: &mut ZipWriter<File>, src_root: &str, options: FileOptions) -> Result<(), Error> {
    tui.update_current_task(format!("Adding path {} to zip", file.display()));
    let relative_name = file.strip_prefix(src_root)?.as_os_str().to_str();
    match relative_name {
        None => Err(new_error_s("Unexpected error in path calculations")),
        Some(relative_name) => {
            zip.add_directory(relative_name, options)?;
            Ok(())
        }
    }
}

pub fn copy_to_zip<S1: AsRef<str>, S2: AsRef<str>>(tui: &mut TUI, src_root_absolute: S1, dirs: Vec<BackupRelPath>, dest_zip_absolute: S2) -> Result<(), Error> {
    let dest_zip = Path::new(dest_zip_absolute.as_ref());
    if dest_zip.exists() {
        return Err(new_error_s(format!("{} already exists!", dest_zip_absolute.as_ref())));
    }
    if !dest_zip_absolute.as_ref().ends_with(".zip") {
        return Err(new_error_s(format!("{} is not a zip file!", dest_zip_absolute.as_ref())));
    }
    let src_root = Path::new(src_root_absolute.as_ref());
    if !src_root.exists() {
        return Err(new_error_s(format!("{} does not exist", src_root_absolute.as_ref())));
    }
    let dest_parent = dest_zip.parent();
    if dest_parent.is_none() {
        return Err(new_error_s(format!("{} is an invalid path", dest_zip_absolute.as_ref())));
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
        let user_specified_dir_to_run_path = src_root.join(&user_specified_dir_to_run.rel_path);
        if !user_specified_dir_to_run_path.exists() {
            return Err(new_error_s(format!("{} does not exist", user_specified_dir_to_run_path.display())));
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
                        zip_one_file_entry(tui, file_or_subdir, &mut zip, &src_root_absolute.as_ref(), options, &mut buffer);
                    } else if (user_specified_dir_to_run.include_subfolders) {
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
    /*
//probably handle if direct file here!
    while let Some(to_run_src_dir) = to_run_src_dirs.pop() {}

    while let Some(current_sub_dir) = to_run_src_dirs.pop() {
        match current_sub_dir.clone().strip_prefix(src_root)?.as_os_str().to_str()
        {
            None => Err(new_error_s("Unexpected error in path. Do you have special characters in any path you want to backup?")),
            Some(relative_name) => {
                if current_sub_dir.is_dir() {
                    for file_or_dir in std::fs::read_dir(current_sub_dir)? {
                        let file_or_dir = file_or_dir?.path();
                        if file_or_dir.is_file() {
                            tui.update_current_task(format!("Zipping {}", file_or_dir.display()));
                            zip.start_file(relative_name, options)?;
                            let mut f = File::open(current_sub_dir)?;

                            f.read_to_end(&mut buffer)?;
                            zip.write_all(&*buffer)?;
                            buffer.clear();
                        }
                    }
                    if src_dir_relative.include_subfolders {
                        tui.update_current_task(format!("Adding path {}", current_sub_dir.display()));

                        to_run_src_dirs.push(current_sub_dir);

                        zip.add_directory(relative_name, options)?;
                    }
                    asdf
                } else {
                    tui.update_current_task(format!("Zipping {}", current_sub_dir.display()));
                    zip.start_file(relative_name, options)?;
                    let mut f = File::open(current_sub_dir)?;

                    f.read_to_end(&mut buffer)?;
                    zip.write_all(&*buffer)?;
                    buffer.clear();
                }
                Ok(())
            }
        }?
    }
}
tui.update_current_task("All entries zipped...");
zip.finish()?;
Ok(())*/
    /*
            let src_dir = src_root.join(&src_dir_relative.rel_path);
            for file_or_dir in std::fs::read_dir()?.into_iter() {}

            for src_item in WalkDir::new(src_dir) {
                let src_item = src_item?;
                let path = src_item.path();
                let name = path.strip_prefix(src_root)?;

                // Write file or directory explicitly
                // Some unzip tools unzip files with directory paths correctly, some do not!
                if path.is_file() {
                    tui.update_current_task(format!("Zipping {}", path.to_str().unwrap_or("?")));
                    zip.start_file_from_path(name, options)?;
                    let mut f = File::open(path)?;

                    f.read_to_end(&mut buffer)?;
                    zip.write_all(&*buffer)?;
                    buffer.clear();
                } else if name.as_os_str().len() != 0 && src_dir_relative.include_subfolders {
                    // Only if not root! Avoids path spec / warning
                    // and mapname conversion failed error on unzip
                    tui.update_current_task(format!("Zipping {}", path.to_str().unwrap_or("?")));
                    zip.add_directory_from_path(name, options)?;
                }
            }
        }*/
}