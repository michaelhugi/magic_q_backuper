use crate::tui::{MenuItem, TUI};

mod console;
mod tui;
mod systems;
mod local_installation;
mod error;
mod zip;
mod zip_name;


fn main() {
    let mut tui = TUI::new();
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
            MenuItem::BackupLocalInstallation(local_installation) => {
                match local_installation.backup(&mut tui) {
                    Ok(success_message) => {
                        tui.show_and_confirm_success(vec![success_message], MenuItem::ChooseBackupSystem)
                    }
                    Err(err) => {
                        tui.show_and_confirm_error(err.texts(), MenuItem::ChooseBackupSystem, true)
                    }
                }
            }
            MenuItem::ExitProgram() => std::process::exit(0),
        }
    }
}
