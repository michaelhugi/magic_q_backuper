use std::path::Path;

use serde::*;

use crate::cmdline::Cmdline;
use crate::console::Console;
use crate::local_pc::LocalPc;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) consoles: Vec<Console>,
    pub(crate) local_pc: Option<LocalPc>,
}


#[derive(Debug, Deserialize, Clone)]
pub(crate) struct BackupRelPath {
    pub(crate) rel_path: String,
    pub(crate) include_subfolders: bool,
}

pub(crate) const EXAMPLE_CONFIG_FILE: &str = r#"{
  "consoles": [
    {
      "name": "My Mq500M",
      "ip": "192.168.0.51",
      "username": "magicQ",
      "password": "magicQ",
      "backup_folders": [
        {
          "folder": "show\\audio",
          "include_subfolders": false
        },
        {
          "folder": "show\\bitmaps",
          "include_subfolders": false
        },
        {
          "folder": "show\\fx",
          "include_subfolders": false
        },
        {
          "folder": "show\\log",
          "include_subfolders": false
        },
        {
          "folder": "show",
          "include_subfolders": false
        }
      ],
      "dest": "K:\\MyBackupfolder\\mq500"
    },
    {
      "name": "My Mq80",
      "ip": "192.168.0.52",
      "username": "magicQ",
      "password": "magicQ",
      "backup_folders": [
        {
          "folder": "show",
          "include_subfolders": true
        }
      ],
      "dest": "K:\\MyBackupfolder\\mq80"
    }
  ],
  "local_pc": {
    "src": "C:\\Users\\michael.hugi\\Documents\\MagicQ",
    "dest": "K:\\MyBackupfolder\\onPc",
    "backup_folders": [
      {
        "folder": "show",
        "include_subfolders": true
      }
    ]
  }
}"#;

pub(crate) fn must_load_config(cmd: &mut Cmdline) -> Config {
    let path = Path::new("mq_backuper_config.json");
    if !path.exists() {
        cmd.write_red("file my_backuper_config.json is missing. Please add file before using the application");
        cmd.end_program(false)
    }
    let config_file = std::fs::read_to_string(path).unwrap_or_else(|_e| {
        cmd.write_red("Could not read file mq_backuper_config.json");
        cmd.end_program(false)
    });
    serde_json::from_str(&config_file).unwrap_or_else(|e1| {
        cmd.write_red(format!("Could not read file mq_backuper_config.json: {}\nExampleConfigFile:\n{}", e1, EXAMPLE_CONFIG_FILE).as_str());
        cmd.end_program(false)
    })
}