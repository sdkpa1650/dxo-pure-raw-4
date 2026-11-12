//! Minimal command-line interface for the Dxo Pure Raw 4 helper binary.

use std::env;

pub enum Command {
    Version,
    Help,
    Install,
    Repair,
    Uninstall,
    Unknown(String),
}

pub fn parse() -> Command {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()).unwrap_or("") {
        "--version" | "-v" => Command::Version,
        "--help"    | "-h" => Command::Help,
        "install"          => Command::Install,
        "repair"           => Command::Repair,
        "uninstall"        => Command::Uninstall,
        other              => Command::Unknown(other.to_string()),
    }
}

pub fn help_text() -> &'static str {
    "Dxo Pure Raw 4 v3.7.6 - Windows helper\n\
     \n\
     USAGE:\n\
         dxo-pure-raw-4-helper <COMMAND>\n\
     \n\
     COMMANDS:\n\
         install    Install or update Dxo Pure Raw 4\n\
         repair     Repair a broken installation\n\
         uninstall  Remove Dxo Pure Raw 4 and its config\n\
         --version  Print version and exit\n\
         --help     Show this help\n"
}
