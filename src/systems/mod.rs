use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::*;

use crate::console::Console;
use crate::error::Error;
use crate::local_installation::LocalInstallation;

#[derive(Debug, Deserialize)]
pub struct Systems {
    pub consoles: Option<Vec<Console>>,
    pub local_installations: Option<Vec<LocalInstallation>>,
}


#[derive(Debug, Deserialize, Clone)]
pub struct BackupRelPath {
    pub excluded_files: Option<Vec<String>>,
    pub rel_path: String,
    pub include_subfolders: bool,
}

pub const CONFIG_FILE_NAME: &str = "config.json";

pub fn get_example_config_file() -> String {
    EXAMPLE_CONFIG_FILE_WITHOUT_UN.replace("{your_username}", whoami::username().as_str())
}

const EXAMPLE_CONFIG_FILE_WITHOUT_UN: &str = r#"{
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
          "excluded_files":[
            "heads.all",
            "*.all"
          ],
          "rel_path": "show",
          "include_subfolders": false
        }
      ],
      "dest": "C:\\PathToYourGoogleDriveFolder"
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
      "dest": "C:\\PathToYourGoogleDriveFolder"
    }
  ],
  "local_installations": [
    {
      "name": "MagicQ on Pc",
      "src": "C:\\Users\\{your_username}\\Documents\\MagicQ",
      "dest": "C:\\PathToYourGoogleDriveFolder",
      "backup_rel_paths": [
        {
          "excluded_files":[
            "heads.all",
            "*.sbk"
          ],
          "rel_path": "show",
          "include_subfolders": false
        },
        {
          "rel_path": "show\\icons\\icon0a00000b.mc2",
          "include_subfolders": true
        }
      ]
    },
    {
      "name": "Capture",
      "src": "D:\\{your_username}\\Documents\\Capture",
      "dest": "C:\\PathToYourGoogleDriveFolder",
      "backup_rel_paths": [
        {
          "rel_path": "",
          "include_subfolders": true
        }
      ]
    }
  ]
}"#;

//Loads the systems from Config.json
fn load_systems() -> Result<Systems, Error> {
    let path = Path::new(&CONFIG_FILE_NAME);
    if !path.exists() {
        return Err(Error::new(vec![
            format!("file {} is missing. Please add file before using the application", &CONFIG_FILE_NAME),
            "Consider looking into the help section for further information".to_string(),
        ]));
    }
    Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
}

//Holds all valid specified consoles local installations and warnings about not valid items
pub struct ValidConsolesAndLocalInstallations {
    pub consoles: Vec<Console>,
    pub local_installations: Vec<LocalInstallation>,
    pub warnings: Vec<Error>,
}

impl ValidConsolesAndLocalInstallations {
    pub fn is_empty(&self) -> bool {
        self.consoles.is_empty() && self.local_installations.is_empty()
    }
}

//Loads all systems from config file, prints errors if available and returns valid entries as well as a list of errors that should just be warnings
pub fn load_validated_consoles_and_local_installations() -> Result<ValidConsolesAndLocalInstallations, Error> {
    match load_systems() {
        Ok(systems) => {
            let mut warnings = Vec::new();
            let mut consoles = Vec::new();
            if systems.consoles.is_some() {
                for console in systems.consoles.unwrap().into_iter() {
                    match console.validate() {
                        Ok(_) => consoles.push(console),
                        Err(e) => warnings.push(e),
                    }
                }
            }
            let mut local_installations = Vec::new();
            if systems.local_installations.is_some() {
                for local_installation in systems.local_installations.unwrap().into_iter() {
                    match local_installation.validate() {
                        Ok(_) => local_installations.push(local_installation),
                        Err(e) => warnings.push(e)
                    }
                }
            }

            Ok(ValidConsolesAndLocalInstallations {
                consoles,
                local_installations,
                warnings,
            })
        }
        Err(err) => {
            Err(Error::new_j(format!("could not read {}", CONFIG_FILE_NAME), err))
        }
    }
}

//Creates a config file for the user with example data
pub fn create_config_json() -> Result<String, Error> {
    let path = Path::new(&CONFIG_FILE_NAME);
    if path.exists() {
        return Err(Error::new_s(format!("{} already exists", &CONFIG_FILE_NAME)));
    }
    File::create(path)?.write_all(get_example_config_file().as_bytes())?;

    Ok(match std::env::current_dir() {
        Ok(dir) => format!("{} created!", dir.join(path).display()),
        Err(_) => format!("{} created!", path.display())
    })
}