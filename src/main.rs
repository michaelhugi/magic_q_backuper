use crate::cmdline::{get_number_input, write_green, write_red};
use crate::console::Console;
use crate::local_pc::LocalPc;

mod config;
mod cmdline;
mod local_pc;
mod console;

fn main() {
    cmdline::starting_menu();
    let config = config::must_load_config();

    let mut valid_console_map = Vec::new();


    for console in config.consoles.into_iter() {
        if console.validate() {
            valid_console_map.push(console);
        }
    }
    let pc_valid = config.local_pc.is_some() && config.local_pc.as_ref().unwrap().validate();

    let local_pc = if pc_valid {
        config.local_pc
    } else {
        None
    };
    choose_console_to_backup(valid_console_map, local_pc);
}

fn choose_console_to_backup(valid_console_map: Vec<Console>, valid_local_pc: Option<LocalPc>) {
    write_green("\n\nChoose which system you want to backup:");
    write_green("0) all listed systems");
    for (index, console) in valid_console_map.iter().enumerate() {
        write_green(format!("{}) {}", index + 1, console.name).as_str())
    }
    let mut pc_index = usize::MAX - 1;
    let mut abort_index = valid_console_map.len() + 1;
    if valid_local_pc.is_some() {
        pc_index = abort_index;
        abort_index += 1;
        write_green(format!("{}) Local Pc", pc_index).as_str())
    }
    write_green(format!("{}) Exit program", abort_index).as_str());
    write_green("");


    let input = cmdline::get_number_input();

    if input == pc_index {
        valid_local_pc.unwrap().backup();
        main();
    } else if input == abort_index {
        std::process::exit(0);
    } else if input == 0 {
        //Backup all systems
        for console in valid_console_map.into_iter() {
            if !console.backup() {
                main();
                return;
            }
        }
        if valid_local_pc.is_some() {
            if !valid_local_pc.unwrap().backup() {
                main();
                return;
            }
        }
        main();
    } else {
        let console = valid_console_map.get(input - 1);
        if console.is_some() {
            let console = console.unwrap();
            if !console.backup() {
                write_red(format!("Could not backup {}", console.name).as_str());
                write_red("Press any key to continue");
                get_number_input();
            } else {
                write_green(format!("Console {} backuped to {}", console.name, console.dest).as_str())
            }
            main();
        } else {
            write_red("Invalid input!");
            choose_console_to_backup(valid_console_map, valid_local_pc);
        }
    }
}

