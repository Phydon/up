// hide console window on Windows in release
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//TODO
// check for command/subcommand/arg conflicts (clap -> conflicts_with)
// add "exlude" command
// add "show outputfile location" command
// symbol to program
//     check if nerd font is set in terminal
//     -> if not print first char, else print symbol if available
pub mod app;
pub mod commands;
pub mod dir_work;
pub mod programs;
use crate::app::up;
use crate::commands::{confirm, get_sys, init, list_programs};
use crate::dir_work::*;
use crate::programs::load_programs;

use colored::*;
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use log::error;

use std::path::Path;
use std::process;

fn main() {
    // handle Ctrl+C
    ctrlc::set_handler(move || {
        println!(
            "{} {}",
            "🤬",
            "Received Ctrl-C! => Exit program!".bold().yellow()
        );
        process::exit(0)
    })
    .expect("Error setting Ctrl-C handler");

    // get tmp dir
    let tmp_dir = check_create_tmp_dir().unwrap_or_else(|err| {
        error!("Unable to find or create a temporary directory: {err}");
        process::exit(1);
    });

    // get config dir
    let config_dir = check_create_config_dir().unwrap_or_else(|err| {
        error!("Unable to find or create a config directory: {err}");
        process::exit(1);
    });

    // initialize the logger
    let _logger = Logger::try_with_str("warn") // log warn and error
        .unwrap()
        .format_for_files(detailed_format) // use timestamp for every log
        .log_to_file(
            FileSpec::default()
                .directory(&config_dir)
                .suppress_timestamp(),
        ) // change directory for logs, no timestamps in the filename
        .append() // use only one logfile
        .duplicate_to_stderr(Duplicate::Info) // print infos, warnings and errors also to the console
        .start()
        .unwrap();

    // set up the programs from config file
    let ron = Path::new(&config_dir).join("up_config.ron");
    let programs = load_programs(&ron).unwrap_or_else(|err| {
        error!("Unable to load programs from {}: {}", ron.display(), err);
        process::exit(1);
    });

    // handle arguments
    let matches = up().get_matches();
    let verbose_flag = matches.get_flag("verbose");
    match matches.subcommand() {
        Some(("clean", _)) => {
            let msg = format!(
                "{}",
                "Do you really want to delete all temporary files? (y/n)"
                    .red()
                    .bold()
            );
            if confirm(&msg) {
                if let Err(err) = remove_tmps(&tmp_dir) {
                    error!("Error while cleaning temporary directory: {}", err);
                    process::exit(1);
                } else {
                    println!("{} {}", "🗑️", "All temporary files removed".bold().red());
                }
            } else {
                println!("Nevermind then");
            }
        }
        Some(("info", sub_match)) => {
            if let Err(err) = init(programs, "info") {
                error!("Error executing cmds: {}", err);
                process::exit(1);
            }
            if sub_match.get_flag("verbose") {
                if let Err(err) = open_tmp("all") {
                    error!("Unable to open output files: {}", err);
                    process::exit(1);
                }
            }
            // TODO add info about one program if program is given after "up info [PROGRAM]"
            // info(sub_matches.get_one::<String>("PROGRAM").expect("required"));
        }
        Some(("log", _)) => {
            if let Ok(logs) = show_log_file(&config_dir) {
                println!("{}", "Available logs:".bold().yellow());
                println!("{}", logs);
            } else {
                error!("Unable to read logs");
                process::exit(1);
            }
        }
        Some(("sys", _)) => {
            get_sys();
        }
        Some(("open", sub_match)) => {
            let arg = sub_match
                .get_one::<String>("PROGRAM")
                .map(|s| s.as_str())
                .expect("required");
            if let Err(err) = open_tmp(arg) {
                error!("Unable to open output files: {}", err);
                process::exit(1);
            }
        }
        Some(("list", _)) => {
            list_programs(&programs);
        }
        // FIXME
        // Some(("exclude", _)) => {
        // let apps: Vec<_> = sub_matches
        //     .get_many::<String>("PROGRAM")
        //     .expect("required")
        //     .map(|s| s.as_str())
        //     .collect();
        // let programs = apps.join(", ");

        // let filtered = exclude(&programs).expect("Error trying to exclude programs");
        // for program in filtered {
        //     println!("{}", program);
        // }

        // if let Err(err) = init(filtered, "update") {
        //     error!("Error executing cmds: {}", err);
        //     process::exit(1);
        // }
        // }
        _ => {
            if let Err(err) = init(programs, "update") {
                error!("Error executing cmds: {}", err);
                process::exit(1);
            }
            if verbose_flag {
                if let Err(err) = open_tmp("all") {
                    error!("Unable to open output files: {}", err);
                    process::exit(1);
                }
            }
        }
    }
}
