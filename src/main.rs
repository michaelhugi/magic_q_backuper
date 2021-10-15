use std::io::Error;

use crate::cmdline::{Cmdline, new_cmdline};
use crate::console::Console;
use crate::local_pc::LocalPc;

mod config;
mod cmdline;
mod local_pc;
mod console;
mod copy;

fn main() {
    let mut cmd = new_cmdline();
    cmd.starting_menu();
    let config = config::must_load_config(&mut cmd);

    let mut valid_console_map = Vec::new();


    for console in config.consoles.into_iter() {
        if console.validate() {
            valid_console_map.push(console);
        }
    }
    let pc_valid = config.local_pc.is_some() && config.local_pc.as_ref().unwrap().validate(&mut cmd);

    let local_pc = if pc_valid {
        config.local_pc
    } else {
        None
    };
    choose_console_to_backup(valid_console_map, local_pc, &mut cmd);
}

fn choose_console_to_backup(valid_console_map: Vec<Console>, valid_local_pc: Option<LocalPc>, cmd: &mut Cmdline) {
    cmd.write_green("\nChoose which system you want to backup:");
    cmd.write_green("0) all listed systems");
    for (index, console) in valid_console_map.iter().enumerate() {
        cmd.write_green(format!("{}) {}", index + 1, console.name).as_str())
    }
    let mut pc_index = usize::MAX - 1;
    let mut abort_index = valid_console_map.len() + 1;
    if valid_local_pc.is_some() {
        pc_index = abort_index;
        abort_index += 1;
        cmd.write_green(format!("{}) Local Pc", pc_index).as_str())
    }
    cmd.write_green(format!("{}) Exit program", abort_index).as_str());
    let home_index = abort_index + 1;
    cmd.write_green(format!("{}) Home", home_index).as_str());

    let input = cmd.get_number_input();

    if input == pc_index {
        match valid_local_pc.unwrap().backup(cmd) {
            Ok(_) => {}
            Err(e) => { cmd.write_red(format!("Could not backup pc system:\n{}", e).as_str()); }
        };
        main();
    } else if input == abort_index {
        std::process::exit(0);
    } else if input == home_index {
        main();
    } else if input == 0 {
        //Backup all systems
        for console in valid_console_map.into_iter() {
            if !console.backup() {
                main();
                return;
            }
        }
        if valid_local_pc.is_some() {
            match valid_local_pc.unwrap().backup(cmd) {
                Ok(_) => {}
                Err(_) => {
                    main();
                    return;
                }
            }
        }
        main();
    } else {
        let console = valid_console_map.get(input - 1);
        if console.is_some() {
            let console = console.unwrap();
            if !console.backup() {
                cmd.write_red(format!("Could not backup {}", console.name).as_str());
                cmd.write_red("Press any key to continue");
                cmd.get_number_input();
            } else {
                cmd.write_green(format!("Console {} backuped to {}", console.name, console.dest).as_str())
            }
            main();
        } else {
            cmd.write_red("Invalid input!");
            choose_console_to_backup(valid_console_map, valid_local_pc, cmd);
        }
    }
}

