use std::{
    io::{stdin, stdout, Write},
};
use std::io::{Stdin, Stdout};

use crossterm::{ExecutableCommand, style::{Color, SetForegroundColor}};
use crossterm::cursor::MoveTo;
use crossterm::style::{Attribute, ResetColor, SetAttribute};
use crossterm::terminal::{Clear, ClearType, enable_raw_mode};

use crate::console::Console;
use crate::error::Error;
use crate::local_installation::LocalInstallation;
use crate::systems::{CONFIG_FILE_NAME, create_config_json, EXAMPLE_CONFIG_FILE, load_validated_consoles_and_local_installations};

pub const SEPARATOR_LINE: &[u8] = "---------------------------------------------------------------------\n".as_bytes();
pub const EMPTY_LINE: &[u8] = "\n".as_bytes();

//Terminal UI
//It has multiple methods to enter a program-part or menu. These parts are blocking, showing the user choices, then the choice is sent back up the tree (so unused variables get dropped) until the main loop to show the next (or same) menu
pub struct TUI {
    stdout: Stdout,
    stdin: Stdin,
}

//New Terminal UI
pub fn new_tui() -> TUI {
    let stdout = stdout();
    // enable_raw_mode().unwrap();
    let stdin = stdin();
    TUI {
        stdout,
        stdin,
    }
}

//All possible entry points to parts of the program
impl TUI {
    //Shows and handles the main menu
    pub fn show_main_menu(&mut self) -> MenuItem {
        self.write_title("Welcome to MagicQ Backuper");
        self.show_menu(vec![MenuItem::Help, MenuItem::ChooseBackupSystem], MenuItem::Home)
    }
    //Shows some help about the program to the user and shows him a menu for more info or going back home
    pub fn show_help(&mut self) -> MenuItem {
        self.write_title("Help");
        self.writeln("The program can simplify the following tasks for you:");
        self.writeln(" - Backing up one or more consoles in the network to a location at your pc");
        self.writeln(" - Backing up one or more pc installations on this computer to another location");
        self.writeln("The pc installations can be other softwares than MagicQ (like Capture or any other software containing information about your show)");
        self.writeln("The backup will be zipped in the destination");
        self.writeln("The destination location is most likely your local folder to google-drive or dropbox so your files get synced to the cloud automatically");
        self.writeln("");
        self.writeln(format!("Note that you need to specify a {} file to the location where this program runs. In this file you specify all the systems that are on this computer or in the network of this computer", CONFIG_FILE_NAME));
        self.show_menu(vec![MenuItem::ShowConfigExample, MenuItem::ShowConfigLocation, MenuItem::CreateConfigExample], MenuItem::Help)
    }
    //Shows the user where the config should be located and shows a menu for next actions
    pub fn show_config_location(&mut self) -> MenuItem {
        self.write_title(format!("Path of {}", CONFIG_FILE_NAME));

        match std::env::current_dir() {
            Ok(path) => {
                self.writeln(format!("You need to store your {} in this folder:", CONFIG_FILE_NAME));
                self.write_successln(format!("{}", path.display()));
            }
            Err(_) => self.write_errorln("Could not read the path of your system")
        }
        self.show_menu(vec![MenuItem::CreateConfigExample, MenuItem::Help], MenuItem::ShowConfigLocation)
    }
    //Shows the user an example config and shows a menu for next actions
    pub fn show_config_example(&mut self) -> MenuItem {
        self.write_title(format!("Example of {}", CONFIG_FILE_NAME));
        self.write_success(EXAMPLE_CONFIG_FILE);
        self.show_menu(vec![MenuItem::Help, MenuItem::CreateConfigExample], MenuItem::ShowConfigExample)
    }

    //Creates a config-file if not present and shows the result to the user, waiting for input and returning to home menu
    pub fn create_config_example(&mut self) -> MenuItem {
        self.write_title(format!("Creating example {}", CONFIG_FILE_NAME));
        match create_config_json() {
            Ok(file) => self.show_and_confirm_success(vec![format!("Created file {}", file)], MenuItem::Home),
            Err(err) => self.show_and_confirm_error(vec![format!("Could not create {}:\n{}", CONFIG_FILE_NAME, err)], MenuItem::Home, false)
        }
    }
    //Shows a list of available systems to the user and lets him choose what system (or all) he wants to backup.
    pub fn show_choose_system_to_backup(&mut self) -> MenuItem {
        self.write_title("Choose system to backup");
        match load_validated_consoles_and_local_installations(self) {
            Ok((consoles, local_installations, warnings)) => {
                if consoles.len() == 0 && local_installations.len() == 0 {
                    return self.show_and_confirm_error(vec![format!("No valid systems found for backup in {}", CONFIG_FILE_NAME), format!("Consider looking in the {} menu", MenuItem::Help.text()), "There may be error messages printed out in the console to help you find what you did wrong".to_string()], MenuItem::Home, false);
                }
                if warnings.len() > 0 {
                    let mut w = Vec::new();
                    for e in warnings.into_iter() {
                        for e in e.texts().into_iter() {
                            w.push(e);
                        }
                        w.push("".to_string());
                    }
                    self.show_and_confirm_warning(w);
                }

                let mut menu = vec![MenuItem::BackupAllSystems(consoles.clone(), local_installations.clone())];

                for console in consoles.into_iter() {
                    menu.push(MenuItem::BackupConsole(console));
                }
                for local_installation in local_installations.into_iter() {
                    menu.push(MenuItem::BackupLocalInstallation(local_installation));
                }
                self.show_menu(menu, MenuItem::ChooseBackupSystem)
            }
            Err(err) => {
                self.show_and_confirm_error(err.texts(), MenuItem::Home, true)
            }
        }
    }

    //Clears the console and then writes a title with separator lines in a constant styling
    pub fn write_title<S: AsRef<str>>(&mut self, text: S) {
        let _ = self.stdout.execute(SetAttribute(Attribute::Reset));
        let _ = self.stdout.execute(Clear(ClearType::Purge));
        let _ = self.stdout.execute(SetAttribute(Attribute::Bold));
        let _ = self.stdout.execute(SetForegroundColor(Color::Blue));
        let _ = self.stdout.flush();
        let _ = self.stdout.write(EMPTY_LINE);
        let _ = self.stdout.write(SEPARATOR_LINE);
        let _ = self.stdout.write(SEPARATOR_LINE);
        let _ = self.stdout.write(format!("     {}\n", text.as_ref().to_uppercase()).as_bytes());
        let _ = self.stdout.write(SEPARATOR_LINE);
        let _ = self.stdout.write(SEPARATOR_LINE);
        let _ = self.stdout.write(EMPTY_LINE);
        let _ = self.stdout.execute(SetAttribute(Attribute::Reset));
    }

    //Simply writes text without newline in standard style and color to the command outpout
    pub fn write<S: AsRef<str>>(&mut self, text: S) {
        let _ = self.stdout.execute(SetAttribute(Attribute::Reset));
        let _ = self.stdout.execute(ResetColor);
        let _ = self.stdout.flush();
        let _ = self.stdout.write(text.as_ref().as_bytes());
    }

    //Simply writes a line in standard style and color to the command outpout
    pub fn writeln<S: AsRef<str>>(&mut self, text: S) {
        let _ = self.stdout.execute(SetAttribute(Attribute::Reset));
        let _ = self.stdout.execute(ResetColor);
        let _ = self.stdout.flush();
        let _ = self.stdout.write(format!("{}\n", text.as_ref()).as_bytes());
    }

    //Writes a line in red to the command outpout
    pub fn write_errorln<S: AsRef<str>>(&mut self, text: S) {
        let _ = self.stdout.execute(SetAttribute(Attribute::Reset));
        let _ = self.stdout.execute(SetForegroundColor(Color::Red));
        let _ = self.stdout.flush();
        let _ = self.stdout.write(format!("ERROR: {}\n", text.as_ref()).as_bytes());
    }
    //Writes a line in red to the command outpout
    pub fn write_success<S: AsRef<str>>(&mut self, text: S) {
        let _ = self.stdout.execute(SetAttribute(Attribute::Reset));
        let _ = self.stdout.execute(SetForegroundColor(Color::Green));
        let _ = self.stdout.flush();
        let _ = self.stdout.write(text.as_ref().as_bytes());
    }
    //Writes a line in green to the command outpout
    pub fn write_successln<S: AsRef<str>>(&mut self, text: S) {
        let _ = self.stdout.execute(SetAttribute(Attribute::Reset));
        let _ = self.stdout.execute(SetForegroundColor(Color::Green));
        let _ = self.stdout.flush();
        let _ = self.stdout.write(format!("{}\n", text.as_ref()).as_bytes());
    }
    //Writes a line in yellow to the command outpout
    pub fn write_warnln<S: AsRef<str>>(&mut self, text: S) {
        let _ = self.stdout.execute(SetAttribute(Attribute::Reset));
        let _ = self.stdout.execute(SetForegroundColor(Color::DarkYellow));
        let _ = self.stdout.flush();
        let _ = self.stdout.write(format!("{}\n", text.as_ref()).as_bytes());
    }

    //Writes process to the cmd output and replaces it's content in the cmd-line so not for every refresh a new line is paintet
    //TODO: make it smarter!
    pub fn write_progress(&mut self, done: &f64, total: &f64, last_percentage: &usize, task: &str) -> usize {
        let _ = self.stdout.execute(SetAttribute(Attribute::Reset));
        let _ = self.stdout.flush();
        let total = if total == &0f64 {
            &1f64
        } else {
            total
        };
        let percentage = (done / total * 100f64) as usize;
        if last_percentage == &percentage {
            return percentage;
        }
        let _ = self.stdout.execute(SetForegroundColor(Color::Green));
        print!("\rProcessing {}%... {}", percentage, task);
        let _ = self.stdout.flush().unwrap();

        return percentage;
    }

    //Shows any generic menu. The current_item will be reused in case there is an invalid input
    fn show_menu(&mut self, mut menu_items: Vec<MenuItem>, current_item: MenuItem) -> MenuItem {
        let _ = self.stdout.execute(SetAttribute(Attribute::Reset));
        let _ = self.stdout.execute(ResetColor);
        let _ = self.stdout.flush();
        let _ = self.stdout.write("\n".as_bytes());
        let _ = self.stdout.execute(SetAttribute(Attribute::Underlined));
        let _ = self.stdout.flush();
        let _ = self.stdout.write("Menu Options\n".as_bytes());
        let _ = self.stdout.execute(SetAttribute(Attribute::Reset));
        let _ = self.stdout.flush();
        let _ = self.stdout.write("\n".as_bytes());
        let _ = self.stdout.execute(SetAttribute(Attribute::Italic));
        let _ = self.stdout.flush();
        for (index, menu_item) in menu_items.iter().enumerate() {
            match menu_item {
                //Added implicitly later
                MenuItem::Home => {}
                //Added implicitly later
                MenuItem::ExitProgram() => {}
                menu_item => { let _ = self.stdout.write(format!("{}) {}\n", index, menu_item.text()).as_bytes()); }
            }
        }
        //Adding exit and main menu implicitly
        let main_menu_index = menu_items.len();
        let mut exit_program_index = main_menu_index + 1;
        match current_item {
            MenuItem::Home => exit_program_index = main_menu_index,
            _ => {
                let _ = self.stdout.execute(SetForegroundColor(Color::Green));
                let _ = self.stdout.write(format!("{}) {}\n", main_menu_index, MenuItem::Home.text()).as_bytes());
            }
        }

        let _ = self.stdout.execute(SetForegroundColor(Color::Yellow));
        let _ = self.stdout.write(format!("{}) {}\n\n", exit_program_index, MenuItem::ExitProgram().text()).as_bytes());
        let _ = self.stdout.execute(ResetColor);

        let _ = self.stdout.write("Waiting for user input...".as_bytes());
        let _ = self.stdout.execute(SetAttribute(Attribute::Reset));
        let mut input = String::new();
        self.stdin.read_line(&mut input).ok().expect("Unexpected program error");
        let input = input.trim().to_string().parse().unwrap_or(usize::MAX);

        if input == exit_program_index {
            std::process::exit(0);
        }
        match current_item {
            MenuItem::Home => {}
            _ => {
                if input == main_menu_index {
                    return MenuItem::Home;
                }
            }
        }

        match menu_items.get(input) {
            None => self.show_and_confirm_error(vec!["Invalid input"], current_item, true),
            Some(_) => menu_items.remove(input)
        }
    }

    //Shows a success message to the screen and waits for the user to press any key until it returns the menu-item you want to
    pub fn show_and_confirm_success<S: AsRef<str>>(&mut self, texts: Vec<S>, menu_item: MenuItem) -> MenuItem {
        let _ = self.stdout.write(EMPTY_LINE);

        for text in texts.iter() {
            let _ = self.write_successln(text);
        }
        self.wait_for_any_key(menu_item)
    }

    //Shows a promenent error message to the screen and waits for the user to press any key until it returns the menu-item you want to
    pub fn show_and_confirm_error<S: AsRef<str>>(&mut self, texts: Vec<S>, menu_item: MenuItem, clear_console_before_print: bool) -> MenuItem {
        if clear_console_before_print {
            let _ = self.stdout.execute(Clear(ClearType::All));
            let _ = self.stdout.execute(MoveTo(0, 0));
        }
        let _ = self.stdout.execute(SetAttribute(Attribute::Reset));
        let _ = self.stdout.execute(SetAttribute(Attribute::Bold));
        let _ = self.stdout.execute(SetForegroundColor(Color::Red));
        let _ = self.stdout.write(SEPARATOR_LINE);
        let _ = self.stdout.write(SEPARATOR_LINE);
        let _ = self.stdout.write("   Error\n".to_uppercase().as_bytes());
        let _ = self.stdout.write(SEPARATOR_LINE);
        let _ = self.stdout.write(SEPARATOR_LINE);
        let _ = self.stdout.write(EMPTY_LINE);
        let _ = self.stdout.execute(SetAttribute(Attribute::Reset));

        for text in texts.iter() {
            let _ = self.write_errorln(text);
        }
        self.wait_for_any_key(menu_item)
    }

    //Shows a list of warnings and wait for user to press enter before continuing
    pub fn show_and_confirm_warning<S: AsRef<str>>(&mut self, texts: Vec<S>) {
        for text in texts.iter() {
            let _ = self.write_warnln(text);
        }
        let _ = self.wait_for_any_key(MenuItem::Home);
    }

    //Prints Press any key to continue and passes the menu_item provided back when the user enters any key
    fn wait_for_any_key(&mut self, menu_item: MenuItem) -> MenuItem {
        let _ = self.stdout.execute(SetAttribute(Attribute::Reset));
        let _ = self.stdout.write(EMPTY_LINE);
        self.writeln("Press enter to continue...");
        let mut buf = String::new();
        self.stdin.read_line(&mut buf);
        menu_item
    }
}

pub enum MenuItem {
    Home,
    Help,
    ShowConfigLocation,
    ShowConfigExample,
    CreateConfigExample,
    ChooseBackupSystem,
    BackupAllSystems(Vec<Console>, Vec<LocalInstallation>),
    BackupConsole(Console),
    BackupLocalInstallation(LocalInstallation),
    ExitProgram(),
}

impl MenuItem {
    fn text(&self) -> String {
        match self {
            MenuItem::Home => "Home".to_string(),
            MenuItem::Help => "Help overview".to_string(),
            MenuItem::ShowConfigLocation => format!("Where should this {} be located?", CONFIG_FILE_NAME),
            MenuItem::CreateConfigExample => format!("Create {} with example data for me", CONFIG_FILE_NAME),
            MenuItem::ChooseBackupSystem => "Backup one ore more systems".to_string(),
            MenuItem::BackupAllSystems(_, _) => "All systems".to_string(),
            MenuItem::BackupConsole(console) => format!("Backup {}", console.name),
            MenuItem::BackupLocalInstallation(local_installation) => format!("Backup {}", local_installation.name),
            MenuItem::ExitProgram() => "End program".to_string(),
            MenuItem::ShowConfigExample => format!("Show example of {}", CONFIG_FILE_NAME)
        }
    }
}