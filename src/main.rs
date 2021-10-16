use crate::tui::{MenuItem, new_tui};

mod console;
mod copy;
mod tui;
mod systems;
mod local_installation;
mod error;


fn main() {
    let mut tui = new_tui();
    let mut current_menu_item = MenuItem::Home;
    loop {
        current_menu_item = match current_menu_item {
            MenuItem::Home => tui.show_main_menu(),
            MenuItem::Help => tui.show_help(),
            MenuItem::ShowConfigLocation => tui.show_config_location(),
            MenuItem::ShowConfigExample => tui.show_config_example(),
            MenuItem::CreateConfigExample => tui.create_config_example(),
            MenuItem::ChooseBackupSystem => tui.show_choose_system_to_backup(),
            MenuItem::BackupAllSystems(_, _) => unimplemented!(),
            MenuItem::BackupConsole(_) => unimplemented!(),
            MenuItem::BackupLocalInstallation(_) => unimplemented!(),
            MenuItem::ExitProgram() => std::process::exit(0),
        }
    }
    tui.show_main_menu();
}
