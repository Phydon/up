// hide console window on Windows in release
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//TODO
// use clap => with args: run, info, exclude, clear etc.
// add info/status/version/clear_tmps/exlude/etc commands to programs
// align "output at ..." when outputfile location is printed
// fix or remove haskel
// override older tmpfiles to save space on disk
//     => use different naming style for tmp file
//     => create up_tmp_dir
use chrono::Local;
use colored::*;
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use log::{error, warn};

use std::{
    env,
    error::Error,
    fs,
    process::{self, Command},
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

// Colors
// darkpurple background, red foreground
const F1: u8 = 255;
const F2: u8 = 46;
const F3: u8 = 95;
const B1: u8 = 41;
const B2: u8 = 0;
const B3: u8 = 25;
// green
const F4: u8 = 10;
const F5: u8 = 255;
const F6: u8 = 169;
// purple
const F7: u8 = 127;
const F8: u8 = 83;
const F9: u8 = 191;

struct Program {
    name: String,
    start_extern: bool,
    has_output: bool,
    outputfile: String,
    update_cmd: Option<String>,
    info_cmd: Option<String>,
}

impl Program {
    fn collect_cmds(
        executer: &str,
        start_extern: bool,
        has_output: bool,
        cmd: Option<&str>,
        outputfile: &str,
    ) -> Option<String> {
        let datetime = Local::now().format("%d%m%Y_%H%M%S_%f").to_string();
        let mut output = String::new();
        output.push_str(outputfile);
        output.push_str("_");
        output.push_str(datetime.as_str());
        output.push_str(".txt");

        let mut collected_cmds = String::new();
        match cmd {
            Some(cmd) => match start_extern {
                true => {
                    collected_cmds.push_str("Start-Process ");
                    collected_cmds.push_str(executer);
                    collected_cmds.push_str(" -ArgumentList '");
                    collected_cmds.push_str(cmd);
                    collected_cmds.push_str("'");
                    if has_output {
                        collected_cmds.push_str("-RedirectStandardOutput ");
                        collected_cmds.push_str(output.as_str());
                    }
                    collected_cmds.push_str(" -WindowStyle Hidden");
                    collected_cmds.push_str(" -Wait");
                    collected_cmds.push_str(";");
                }
                false => {
                    collected_cmds.push_str(executer);
                    collected_cmds.push_str(" ");
                    collected_cmds.push_str(cmd);
                    collected_cmds.push_str(";");
                }
            },
            None => return None,
        }

        Some(collected_cmds)
    }

    pub fn new(
        name: &str,
        executer: &str,
        start_extern: bool,
        has_output: bool,
        cmd_for_update: Option<&str>,
        cmd_for_info: Option<&str>,
    ) -> Program {
        let mut tmp = String::new();
        match env::temp_dir().as_os_str().to_str() {
            Some(dir) => {
                tmp.push_str(dir);
            }
            None => {
                let err_msg = "Can`t find temp directory";
                warn!("{err_msg}");
                // FIXME panics if dir already exists
                // TODO check dir.exists()
                let dir = "~/up_tmp/";
                fs::create_dir(dir).expect("Unable to find or create tmp dir");
                tmp.push_str(dir);
            }
        }
        tmp.push_str("up_output_");
        tmp.push_str(name);
        let outputfile = tmp.to_string();

        let update_cmd = Self::collect_cmds(
            executer,
            start_extern,
            has_output,
            cmd_for_update,
            &outputfile,
        );
        let info_cmd = Self::collect_cmds(
            executer,
            start_extern,
            has_output,
            cmd_for_info,
            &outputfile,
        );

        let name = name.to_string();

        Program {
            name,
            start_extern,
            has_output,
            outputfile,
            update_cmd,
            info_cmd,
        }
    }
}

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
    let scoop = Program::new(
        "scoop",
        "powershell",
        true,
        true,
        Some("-c scoop update --all"),
        Some("-c scoop status"),
    );
    let winget = Program::new("winget", "winget", true, true, Some("upgrade"), None);
    let rust = Program::new("rust", "rustup", true, true, Some("update"), None);
    // FIXME or remove
    let haskell = Program::new("haskel", "ghcup", true, true, Some("update"), None);
    let vim = Program::new("vim", "vim", true, false, Some("-c PlugUpdate -c qa"), None);
    let nvim = Program::new(
        "nvim",
        "nvim",
        true,
        false,
        Some("-c PlugUpdate -c qa"),
        None,
    );
    let pip = Program::new(
        "pip",
        "py",
        true,
        true,
        Some("-m pip install --upgrade pip"),
        None,
    );

    let commands: Vec<Program> = vec![scoop, winget, rust, haskell, vim, nvim, pip];

    if let Err(err) = update(commands) {
        error!("Error executing cmds: {}", err);
        process::exit(1);
    }
}

fn update(commands: Vec<Program>) -> Result<(), Box<dyn Error>> {
    println!("{}", "::: STARTING UPDATE".bold().truecolor(F7, F8, F9));

    let num = commands.len() as u64;
    progress_bar(commands, num)?;

    Ok(())
}

fn run_cmd(cmd: &str) -> Result<(), Box<dyn Error>> {
    if cfg!(target_os = "windows") {
        Command::new("powershell").args(["-c", cmd]).status()?
    } else {
        unimplemented!();
        // Command::new("sh")
        //     .arg("-c")
        //     .arg("echo 'not implemented yet'")
        //     .status()?
    };

    Ok(())
}

fn progress_bar(commands: Vec<Program>, num: u64) -> Result<Arc<MultiProgress>, Box<dyn Error>> {
    let started = Instant::now();
    let spinner_style =
        ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}").unwrap();
    // .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    let m = Arc::new(MultiProgress::new());
    let sty = ProgressStyle::with_template(
        "{spinner:.blue} [{elapsed_precise}] {bar:40.red/blue} {pos:>5}/{len:5} {eta:5} {msg}",
    )
    .unwrap()
    // .progress_chars("#>-");
    .progress_chars("=>-");

    let pb = m.add(ProgressBar::new(num));
    pb.set_style(sty);

    pb.tick();
    let handles: Vec<_> = commands
        .into_iter()
        .map(|arg| {
            let pb = pb.clone();
            let spinner = m.add(ProgressBar::new_spinner());
            spinner.enable_steady_tick(Duration::from_millis(200));
            spinner.set_style(spinner_style.clone());
            spinner.set_prefix(format!("[..]"));
            thread::spawn(move || {
                spinner.set_message(format!("{} {}", "updating".truecolor(F7, F8, F9), arg.name));
                spinner.tick();
                match arg.update_cmd {
                    Some(cmd) => {
                        run_cmd(cmd.as_str()).unwrap();
                    }
                    None => {}
                }
                match arg.info_cmd {
                    Some(cmd) => {
                        run_cmd(cmd.as_str()).unwrap();
                    }
                    None => {}
                }
                spinner.finish_with_message(match arg.start_extern {
                    true => match arg.has_output {
                        true => {
                            format!(
                                "{} {}  => output at \"{}\"",
                                arg.name.truecolor(F7, F8, F9),
                                "done".truecolor(F4, F5, F6),
                                arg.outputfile.italic(),
                            )
                        }
                        false => {
                            format!(
                                "{} {}",
                                arg.name.truecolor(F7, F8, F9),
                                "done".truecolor(F4, F5, F6)
                            )
                        }
                    },
                    false => {
                        format!(
                            "{} {}",
                            arg.name.truecolor(F7, F8, F9),
                            "done".truecolor(F4, F5, F6)
                        )
                    }
                });
                pb.inc(1);
            })
        })
        .collect();

    for h in handles {
        let _ = h.join();
    }

    pb.finish_with_message(format!("{}", "done".bold().truecolor(F4, F5, F6)));

    // m.clear().unwrap();

    println!(
        "{} {}",
        "::: ALL UPDATED IN ".bold().truecolor(F4, F5, F6),
        HumanDuration(started.elapsed())
            .to_string()
            .to_uppercase()
            .bold()
            .truecolor(F1, F2, F3)
            .on_truecolor(B1, B2, B3)
    );

    Ok(m)
}
