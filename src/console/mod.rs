use serde::*;

use crate::cmdline::Cmdline;
use crate::config::BackupRelPath;

#[derive(Debug, Deserialize)]
pub(crate) struct Console {
    pub(crate) name: String,
    pub(crate) ip: String,
    pub(crate) backup_rel_paths: Vec<BackupRelPath>,
    pub(crate) dest: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Console {
    pub(crate) fn validate(&self) -> bool {
        true
    }
    pub(crate) fn backup(&self) -> bool { return false; }

    //Returns true if program can continue or false if going back to main
    fn show_error_message(&self, cmd: &mut Cmdline) -> bool {
        cmd.write_red(format!("Could not backup {}", self.name).as_str());
        cmd.write_red("1) Abort");
        cmd.write_red("2) Continue");

        match cmd.get_number_input() {
            1 => false,
            2 => true,
            _ => {
                cmd.write_red("Invalid input");
                self.show_error_message(cmd)
            }
        }
    }
}