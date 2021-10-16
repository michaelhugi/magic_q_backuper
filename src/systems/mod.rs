use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::*;

use crate::console::Console;
use crate::error::{Error, new_error};
use crate::local_installation::LocalInstallation;
use crate::tui::TUI;

#[derive(Debug, Deserialize)]
pub struct Systems {
    pub consoles: Option<Vec<Console>>,
    pub local_installations: Option<Vec<LocalInstallation>>,
}


#[derive(Debug, Deserialize, Clone)]
pub struct BackupRelPath {
    pub rel_path: String,
    pub include_subfolders: bool,
}

pub const CONFIG_FILE_NAME: &str = "config.json";

pub const EXAMPLE_CONFIG_FILE: &str = r#"{
  "consoles": [
    {
      "name": "My Mq500M",
      "ip": "192.168.0.51",
      "username": "magicQ",
      "password": "magicQ",
      "backup_rel_paths": [
        {
          "rel_path": "show\\audio",
          "include_subfolders": false
        },
        {
          "rel_path": "show\\bitmaps",
          "include_subfolders": false
        },
        {
          "rel_path": "show\\fx",
          "include_subfolders": false
        },
        {
          "rel_path": "show\\log",
          "include_subfolders": false
        },
        {
          "rel_path": "show",
          "include_subfolders": false
        }
      ],
      "dest": "C:\\TestbackupMq"
    },
    {
      "name": "My Mq80",
      "ip": "192.168.0.52",
      "username": "magicQ",
      "password": "magicQ",
      "backup_rel_paths": [
        {
          "rel_path": "show",
          "include_subfolders": true
        }
      ],
      "dest": "C:\\TestbackupMq"
    }
  ],
  "local_installations": [
    {
        "src": "C:\\Users\\michael.hugi\\Documents\\MagicQ",
        "dest": "C:\\TestbackupMq",
        "backup_rel_paths": [
        {
            "rel_path": "show",
            "include_subfolders": true
        },
        {
            "rel_path": "show\\icons\\icon0a00000b.mc2",
            "include_subfolders": true
        }
        ]
    },
    {
        "src": "D:\\mhugi\\Documents\\Capture",
        "dest": "C:\\TestbackupMq",
        "backup_rel_paths": [
        {
            "rel_path": "",
            "include_subfolders": true
        }
        ]
    },
  ]
}"#;

pub fn must_load_systems(tui: &mut TUI) -> Systems {
    let path = Path::new(&CONFIG_FILE_NAME);
    if !path.exists() {
        tui.write_errorln(format!("file {} is missing. Please add file before using the application", &CONFIG_FILE_NAME));
        tui.write_warnln("Consider looking into the help section for further information");
        tui.show_main_menu();
    }

    let config_file = std::fs::read_to_string(path).unwrap_or_else(|e| {
        tui.write_errorln(format!("Could not read file {}:\n{}", &CONFIG_FILE_NAME, e));
        tui.show_main_menu();
        panic!("Unexpected end of program");
    });

    serde_json::from_str(&config_file).unwrap_or_else(|e| {
        tui.write_errorln(format!("File {} is bad formatted: {}", &CONFIG_FILE_NAME, e));
        tui.write_warnln("Consider looking into the help section for further information");
        tui.show_main_menu();
        panic!("Unexpected end of program");
    })
}

pub fn load_validated_consoles_and_local_installations(tui: &mut TUI) -> (Vec<Console>, Vec<LocalInstallation>) {
    let systems = must_load_systems(tui);

    let mut consoles = Vec::new();
    if systems.consoles.is_some() {
        for console in systems.consoles.unwrap().into_iter() {
            if console.validate() {
                consoles.push(console)
            }
        }
    }

    let mut local_installations = Vec::new();
    if systems.local_installations.is_some() {
        for local_installation in systems.local_installations.unwrap().into_iter() {
            if local_installation.validate(tui) {
                local_installations.push(local_installation);
            }
        }
    }

    (consoles, local_installations)
}

pub fn create_config_json() -> Result<String, Error> {
    let path = Path::new(&CONFIG_FILE_NAME);
    if path.exists() {
        return Err(new_error(format!("{} already exists", &CONFIG_FILE_NAME)));
    }
    File::create(path)?.write_all(EXAMPLE_CONFIG_FILE.as_bytes())?;

    Ok(match std::env::current_dir() {
        Ok( dir) => format!("{} created!", dir.join(path).display()),
        Err(_) => format!("{} created!", path.display())
    })
}