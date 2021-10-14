use serde::*;

use crate::cmdline::{get_number_input, write_red};
use crate::config::BackupFolder;

#[derive(Debug, Deserialize)]
pub(crate) struct Console {
    pub(crate) name: String,
    pub(crate) ip: String,
    pub(crate) backup_folders: Vec<BackupFolder>,
    pub(crate) dest: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Console {
    pub(crate) fn validate(&self) -> bool {
        true
    }
    pub(crate) fn backup(&self) -> bool { self.show_error_message() }

    //Returns true if program can continue or false if going back to main
    fn show_error_message(&self) -> bool {
        write_red(format!("Could not backup {}", self.name).as_str());
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