// hide console window on Windows in release
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//TODO
// add /exlude/open_tmp_files/etc commands to programs
// align "output at ..." when outputfile location is printed
// check if "ghcup update" works properly
// let user confirm before cleaning tmp files
pub mod app;
pub mod commands;
pub mod dir_work;
pub mod programs;
use crate::app::up;
use crate::commands::init;
use crate::dir_work::{check_create_dir, remove_tmps};
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

    // get tmp dir
    let tmp_dir = check_create_dir().unwrap_or_else(|err| {
        error!("Unable to find or create a temporary directory: {err}");
        process::exit(1);
    });

    // set up the programs
    let scoop = Program::new(
        "scoop",
        "powershell",
        true,
        true,
        Some("-c scoop update --all"),
        Some("-c scoop status"),
        &tmp_dir,
    );
    let winget = Program::new(
        "winget",
        "winget",
        true,
        true,
        Some("upgrade"),
        Some("--info"),
        &tmp_dir,
    );
    let rust = Program::new(
        "rust",
        "rustup",
        true,
        true,
        Some("update"),
        Some("check"),
        &tmp_dir,
    );
    let haskell = Program::new(
        "haskel",
        "ghcup",
        true,
        true,
        Some("update"),
        Some("list"),
        &tmp_dir,
    );
    let vim = Program::new(
        "vim",
        "vim",
        true,
        false,
        Some("-c PlugUpdate -c qa"),
        None,
        &tmp_dir,
    );
    let nvim = Program::new(
        "nvim",
        "nvim",
        true,
        false,
        Some("-c PlugUpdate -c qa"),
        None,
        &tmp_dir,
    );
    let pip = Program::new(
        "pip",
        "py",
        true,
        true,
        Some("-m pip install --upgrade pip"),
        Some("-m pip check"),
        &tmp_dir,
    );

    let commands: Vec<Program> = vec![scoop, winget, rust, haskell, vim, nvim, pip];

    // for testing
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
    //     "test2",
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
        info!("Received Ctrl-C! => Exit program!");
        process::exit(0)
    })
    .expect("Error setting Ctrl-C handler");

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
            }
        }
        Some(("info", _)) => {
            // TODO add info about one program if program is given after "up info [PROGRAM]"
            // info(sub_matches.get_one::<String>("PROGRAM").expect("required"));
            if let Err(err) = init(commands, "info") {
                error!("Error executing cmds: {}", err);
                process::exit(1);
            }
        }
        // Some(("open", sub_matches)) => {
        //     todo!();
        // }
        // Some(("exclude", sub_matches)) => {
        //     todo!();
        // let apps: Vec<_> = sub_matches
        //     .get_many::<String>("PROGRAM")
        //     .expect("required")
        //     .map(|s| s.as_str())
        //     .collect();
        // let programs = apps.join(", ");
        // exclude(programs);
        // }
        _ => unreachable!(),
    }
}
