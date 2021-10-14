use std::io::stdin;
use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::config;

pub(crate) fn write_green(text: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    if stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).is_err() {
        println!("{}", text);
        return;
    }
    if writeln!(&mut stdout, "{}", text).is_err() {
        println!("{}", text)
    }
}

pub(crate) fn write_yellow(text: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    if stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).is_err() {
        println!("{}", text);
        return;
    }
    if writeln!(&mut stdout, "{}", text).is_err() {
        println!("{}", text)
    }
}

pub(crate) fn write_red(text: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    if stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).is_err() {
        println!("{}", text);
        return;
    }
    if writeln!(&mut stdout, "{}", text).is_err() {
        println!("{}", text)
    }
}


pub(crate) fn end_program(success: bool) -> ! {
    if success {
        write_green("Press any key to exit program")
    } else {
        write_yellow("Press any key to exit program");
    }
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).ok().expect("Unexpected program error");
    if success {
        std::process::exit(-1)
    }
    std::process::abort()
}


pub(crate) fn starting_menu() {
    write_green("\nWelcome to MagicQ backuper. Please choose:");
    write_green("1) Help");
    write_green("2) Backup some MQ Systems");
    write_green("3) Exit");

    match get_number_input() {
        1 => show_help(),
        2 => {}
        3 => std::process::exit(0),
        _ => {
            write_red("Invalid input");
            starting_menu();
        }
    }
}

pub(crate) fn get_number_input() -> usize {
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).ok().expect("Unexpected program error");
    input_string.trim().to_string().parse().unwrap_or(usize::MAX)
}

fn show_help() {
    write_green("");
    write_green("---------------------------------------------------------------------");
    write_green("Mq Backuper");
    write_green("---------------------------------------------------------------------");
    write_green("");
    write_green("With this program you can extract some files from an MagicQ console and/or your MagicQ Pc installation, zip them and save the zip to a specified location like dropbox or google-drive for backing up");
    write_green("To use the program, you need to add a file named mq_backuper_config.json to the same location where your program is executed");
    write_green(format!("Here is an example how the config can look like:\n\n{}\n\n", config::EXAMPLE_CONFIG_FILE).as_str());
    write_green("");
    write_green("---------------------------------------------------------------------");
    write_green("");
    starting_menu();
}