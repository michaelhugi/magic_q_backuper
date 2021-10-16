extern crate walkdir;
extern crate zip;

use std::fs::{create_dir_all, File};
use std::io::{Seek, Write};
use std::io::prelude::*;
use std::iter::Iterator;
use std::path::Path;

use walkdir::{DirEntry, WalkDir};
use zip::result::ZipError;
use zip::write::FileOptions;

use crate::error::{Error, new_error_s};
use crate::systems::BackupRelPath;
use crate::tui::TUI;

use self::zip::CompressionMethod;

fn zip_dir<T>(it: &mut Iterator<Item=DirEntry>, prefix: &str, writer: T, method: zip::CompressionMethod)
              -> zip::result::ZipResult<()>
    where T: Write + Seek
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {:?} as {:?} ...", path, name);
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if name.as_os_str().len() != 0 {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {:?} as {:?} ...", path, name);
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}


pub fn create_zip(src_dir: &str, dst_file: &str) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(src_dir.to_string());
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, zip::CompressionMethod::Bzip2)?;

    Ok(())
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

    for src_dir_relative in dirs.iter() {
        let src_dir = src_root.join(&src_dir_relative.rel_path);

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
    }
    tui.update_current_task("All entries zipped...");
    zip.finish()?;
    Ok(())
}