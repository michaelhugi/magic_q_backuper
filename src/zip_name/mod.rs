use std::path::{Path, PathBuf};

pub fn get_zip_path(system_name: &str, dest_dir: &Path) -> PathBuf {
    let now = chrono::offset::Local::now().format("%Y_%m_%d__%H_%M_%S");
    dest_dir.join(format!("{}_backup_{}.zip", system_name, now))
}