// hide console window on Windows in release
// #![windows_subsystem = "windows"]

use colored::*;
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use log::error;

use std::{
    error::Error,
    process::{self, Command},
};

fn main() {
    // initialize the logger
    let _logger = Logger::try_with_str("info") // log info, warn and error
        .unwrap()
        .format_for_files(detailed_format) // use timestamp for every log
        .log_to_file(FileSpec::default().suppress_timestamp()) // no timestamps in the filename
        .append() // use only one logfile
        .duplicate_to_stderr(Duplicate::Info) // print infos, warnings and errors also to the console
        .start()
        .unwrap();

    let commands = vec![
        format!("echo \"{}\"", "::: STARTING UPDATE".bold().yellow()),
        format!("echo \"{}\"", "::: updating scoop ...".cyan()),
        "scoop update".to_string(),
        "scoop status".to_string(),
        format!("echo \"{}\"", "::: updating winget ...".cyan()),
        "winget upgrade".to_string(),
        format!("echo \"{}\"", "::: updating rust ...".cyan()),
        "rustup --verbose update".to_string(),
        format!("echo \"{}\"", "::: updating vim ...".cyan()),
        "vim -c PlugUpdate -c qa".to_string(),
        format!("echo \"{}\"", "::: updating nvim ...".cyan()),
        "nvim -c PlugUpdate -c qa".to_string(),
        format!("echo \"{}\"", "::: updating haskell ...".cyan()),
        "ghcup --verbose upgrade".to_string(),
        format!("echo \"{}\"", "::: DONE".bold().green()),
        // "Start-sleep -seconds 5".to_string(),
    ];

    if let Err(err) = run_cmd(commands) {
        error!("Error executing cmds: {}", err);
        process::exit(1);
    }
}

fn run_cmd(commands: Vec<String>) -> Result<(), Box<dyn Error>> {
    let args: String = collect_args(commands)?;
    cmd(args.as_str())?;

    Ok(())
}

fn cmd(args: &str) -> Result<(), Box<dyn Error>> {
    if cfg!(target_os = "windows") {
        Command::new("powershell").args(["-c", args]).status()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo 'not implemented yet'")
            .status()?
    };

    Ok(())
}

fn collect_args(args_list: Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut combiner = String::new();
    for arg in args_list {
        combiner.push_str(&arg);
        combiner.push_str("; ");
    }

    Ok(combiner)
}
