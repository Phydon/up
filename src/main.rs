// hide console window on Windows in release
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//TODO
// use clap => with args:
//     run, info, exclude, clear, open_tmp_file, etc.
// add info/status/version/clear_tmps/exlude/open/etc commands to programs
// align "output at ..." when outputfile location is printed
// fix or remove haskel
// override older tmpfiles to save space on disk
//     => use different naming style for tmp file
//     => create up_tmp_dir
pub mod app;
pub mod commands;
pub mod dir_work;
pub mod programs;
use crate::app::up;
use crate::commands::update;
use crate::programs::Program;

use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use log::{error, info};

use std::process;

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

    // set up the programs
    // let scoop = Program::new(
    //     "scoop",
    //     "powershell",
    //     true,
    //     true,
    //     Some("-c scoop update --all"),
    //     Some("-c scoop status"),
    // );
    // let winget = Program::new("winget", "winget", true, true, Some("upgrade"), None);
    // let rust = Program::new("rust", "rustup", true, true, Some("update"), None);
    // // FIXME or remove
    // let haskell = Program::new("haskel", "ghcup", true, true, Some("update"), None);
    // let vim = Program::new("vim", "vim", true, false, Some("-c PlugUpdate -c qa"), None);
    // let nvim = Program::new(
    //     "nvim",
    //     "nvim",
    //     true,
    //     false,
    //     Some("-c PlugUpdate -c qa"),
    //     None,
    // );
    // let pip = Program::new(
    //     "pip",
    //     "py",
    //     true,
    //     true,
    //     Some("-m pip install --upgrade pip"),
    //     None,
    // );

    // let commands: Vec<Program> = vec![scoop, winget, rust, haskell, vim, nvim, pip];

    // for testing
    let test1 = Program::new(
        "test1",
        "powershell",
        false,
        false,
        Some("-c Start-Sleep -Seconds 3"),
        None,
    );
    let test2 = Program::new(
        "test2",
        "powershell",
        false,
        false,
        Some("-c Start-Sleep -Seconds 5"),
        None,
    );
    let commands: Vec<Program> = vec![test1, test2];

    // handle Ctrl+C
    ctrlc::set_handler(move || {
        info!("Received [ Ctrl-C ]! Quit program!");
        process::exit(0)
    })
    .expect("Error setting Ctrl-C handler");

    let matches = up().get_matches();
    match matches.subcommand() {
        Some(("run", _)) => {
            if let Err(err) = update(commands) {
                error!("Error executing cmds: {}", err);
                process::exit(1);
            }
        }
        Some(("clean", _)) => {
            todo!();
        }
        Some(("open", sub_matches)) => {
            todo!();
        }
        Some(("info", sub_matches)) => {
            todo!();
            // info(sub_matches.get_one::<String>("APP").expect("required"));
        }
        Some(("exclude", sub_matches)) => {
            todo!();
            // let apps: Vec<_> = sub_matches
            //     .get_many::<String>("APP")
            //     .expect("required")
            //     .map(|s| s.as_str())
            //     .collect();
            // let programs = apps.join(", ");
            // exclude(programs);
        }
        _ => unreachable!(),
    }
}
