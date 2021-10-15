use std::{
    io::{stdin, stdout, Write},
};
use std::io::{Stdin, Stdout};

use crossterm::{ExecutableCommand, style::{Color, Print, SetForegroundColor}};

use crate::{choose_console_to_backup, config};

pub(crate) const SEPARATOR_LINE: &str = "---------------------------------------------------------------------";

pub(crate) struct Cmdline {
    stdout: Stdout,
    stdin: Stdin,
}

pub(crate) fn new_cmdline() -> Cmdline {
    let stdout = stdout();
    let stdin = stdin();
    Cmdline {
        stdout,
        stdin,
    }
}

impl Cmdline {
    pub(crate) fn write_green(&mut self, text: &str) {
        let _ = self.stdout.execute(SetForegroundColor(Color::Green));
        let _ = self.stdout.execute(Print(format!("{}\n", text)));
    }

    pub(crate) fn write_percentage(&mut self, done: &f64, total: &f64, last_percentage: &usize, task: &str) -> usize {
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

    pub(crate) fn write_yellow(&mut self, text: &str) {
        let _ = self.stdout.execute(SetForegroundColor(Color::Yellow));
        let _ = self.stdout.execute(Print(text));
    }

    pub(crate) fn write_red(&mut self, text: &str) {
        let _ = self.stdout.execute(SetForegroundColor(Color::Red));
        let _ = self.stdout.execute(Print(text));
    }

    pub(crate) fn end_program(&mut self, success: bool) -> ! {
        if success {
            self.write_green("Press any key to exit program")
        } else {
            self.write_yellow("Press any key to exit program");
        }
        let mut input_string = String::new();
        self.stdin.read_line(&mut input_string).ok().expect("Unexpected program error");
        if success {
            std::process::exit(-1)
        }
        std::process::abort()
    }


    pub(crate) fn starting_menu(&mut self) {
        self.write_green("\nWelcome to MagicQ backuper. Please choose:");
        self.write_green("1) Help");
        self.write_green("2) Backup some MQ Systems");
        self.write_green("3) Exit");

        match self.get_number_input() {
            1 => self.show_help(),
            2 => choose_console_to_backup(self),
            3 => std::process::exit(0),
            _ => {
                self.write_red("Invalid input");
                self.starting_menu();
            }
        }
    }

    pub(crate) fn get_number_input(&mut self) -> usize {
        let _ = self.stdout.execute(SetForegroundColor(Color::Green));
        let _ = self.stdout.execute(Print("Waiting for user input..."));
        let mut input_string = String::new();
        self.stdin.read_line(&mut input_string).ok().expect("Unexpected program error");
        self.write_green("");
        input_string.trim().to_string().parse().unwrap_or(usize::MAX)
    }


    fn show_help(&mut self) {
        self.write_green("");
        self.write_green(SEPARATOR_LINE);
        self.write_green("Mq Backuper");
        self.write_green(SEPARATOR_LINE);
        self.write_green("");
        self.write_green("With this program you can extract some files from an MagicQ console and/or your MagicQ Pc installation, zip them and save the zip to a specified location like dropbox or google-drive for backing up");
        self.write_green("To use the program, you need to add a file named mq_backuper_config.json to the same location where your program is executed");
        self.write_green(format!("Here is an example how the config can look like:\n\n{}\n\n", config::EXAMPLE_CONFIG_FILE).as_str());
        self.write_green("");
        self.write_green("---------------------------------------------------------------------");
        self.write_green("");
        self.starting_menu();
    }
}