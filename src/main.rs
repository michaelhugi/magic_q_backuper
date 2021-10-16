use crate::tui::{MenuItem, TUI};

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
            MenuItem::BackupAllSystems(local_installations) => {
                let mut successes = Vec::new();
                let mut errors = Vec::new();
                for local_installation in local_installations.into_iter() {
                    match local_installation.backup(&mut tui) {
                        Ok(success_message) => {
                            successes.push(success_message);
                        }
                        Err(err) => {
                            for e in err.texts().into_iter() {
                                errors.push(e);
                            }
                        }
                    }
                }
                if successes.is_empty() {
                    tui.show_and_confirm_error(errors, MenuItem::ChooseBackupSystem, true)
                } else if errors.is_empty() {
                    tui.show_and_confirm_success(successes, MenuItem::ChooseBackupSystem)
                } else {
                    let _ = tui.show_and_confirm_error(errors, MenuItem::ChooseBackupSystem, true);
                    tui.show_and_confirm_success(successes, MenuItem::ChooseBackupSystem)
                }
            }
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
