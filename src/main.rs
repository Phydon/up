// hide console window on Windows in release
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//TODO
// align output
// add /exlude/open_tmp_files/etc commands to programs
// check if "ghcup update" works properly
// let user confirm before cleaning tmp files
pub mod app;
pub mod commands;
pub mod dir_work;
pub mod programs;
use crate::app::up;
use crate::commands::{exclude, init};
use crate::dir_work::{check_create_dir, remove_tmps};
use crate::programs::Program;

use colored::*;
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use log::error;

use std::process;

fn main() {
    // initialize the logger
    let _logger = Logger::try_with_str("warn") // log warn and error
        .unwrap()
        .format_for_files(detailed_format) // use timestamp for every log
        .log_to_file(FileSpec::default().suppress_timestamp()) // no timestamps in the filename
        .append() // use only one logfile
        .duplicate_to_stderr(Duplicate::Info) // print infos, warnings and errors also to the console
        .start()
        .unwrap();

    // get tmp dir
    let tmp_dir = check_create_dir().unwrap_or_else(|err| {
        error!("Unable to find or create a temporary directory: {err}");
        process::exit(1);
    });

    // set up the programs
    let scoop = Program::new(
        "scoop",
        None,
        "powershell",
        true,
        true,
        Some("-c scoop update --all"),
        Some("-c scoop status"),
        &tmp_dir,
    );
    let winget = Program::new(
        "winget",
        None,
        "winget",
        true,
        true,
        Some("upgrade"),
        Some("--info"),
        &tmp_dir,
    );
    let rust = Program::new(
        "rust",
        Some(""),
        "rustup",
        true,
        true,
        Some("update"),
        Some("check"),
        &tmp_dir,
    );
    let haskell = Program::new(
        "haskell",
        Some(""),
        "ghcup",
        true,
        true,
        Some("update"),
        Some("list"),
        &tmp_dir,
    );
    let vim = Program::new(
        "vim",
        None,
        "vim",
        true,
        false,
        Some("-c PlugUpdate -c qa"),
        None,
        &tmp_dir,
    );
    let nvim = Program::new(
        "neovim",
        None,
        "nvim",
        true,
        false,
        Some("-c PlugUpdate -c qa"),
        None,
        &tmp_dir,
    );
    let pip = Program::new(
        "pip",
        Some(""),
        "py",
        true,
        true,
        Some("-m pip install --upgrade pip"),
        Some("-m pip check"),
        &tmp_dir,
    );

    let mut commands: Vec<Program> = vec![haskell, nvim, pip, rust, scoop, vim, winget];

    // for testing
    // println!("{}", "Testing activated".italic().yellow());
    // let test1 = Program::new(
    //     "test1",
    //     "powershell",
    //     true,
    //     true,
    //     Some("-c Start-Sleep -Seconds 3"),
    //     None,
    //     &tmp_dir,
    // );
    // let test2 = Program::new(
    //     "testing2",
    //     "powershell",
    //     false,
    //     false,
    //     Some("-c Start-Sleep -Seconds 5"),
    //     None,
    //     &tmp_dir,
    // );
    // let commands: Vec<Program> = vec![test1, test2];

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

    // handle input args
    let matches = up().get_matches();
    match matches.subcommand() {
        Some(("run", _)) => {
            if let Err(err) = init(commands, "update") {
                error!("Error executing cmds: {}", err);
                process::exit(1);
            }
        }
        Some(("clean", _)) => {
            if let Err(err) = remove_tmps(&tmp_dir) {
                error!("Error while cleaning temporary directory: {}", err);
                process::exit(1);
            } else {
                println!("{} {}", "🗑️", "All temporary files removed".bold().red());
            }
        }
        Some(("info", _)) => {
            if let Err(err) = init(commands, "info") {
                error!("Error executing cmds: {}", err);
                process::exit(1);
            }
            // TODO add info about one program if program is given after "up info [PROGRAM]"
            // info(sub_matches.get_one::<String>("PROGRAM").expect("required"));
        }
        // Some(("open", sub_matches)) => {
        //     todo!();
        // }
        Some(("exclude", _)) => {
            // let apps: Vec<_> = sub_matches
            //     .get_many::<String>("PROGRAM")
            //     .expect("required")
            //     .map(|s| s.as_str())
            //     .collect();
            // let programs = apps.join(", ");
            let included_commands = exclude(commands);
            if let Err(err) = init(included_commands, "update") {
                error!("Error executing cmds: {}", err);
                process::exit(1);
            }
        }
        _ => unreachable!(),
    }
}
