use colored::*;
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use log::error;

use std::{
    error::Error,
    process::{self, Command},
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

struct Program {
    name: String,
    start_extern: bool,
    collected_cmds: String,
}

impl Program {
    pub fn new(name: &str, executer: &str, start_extern: bool, cmds: Vec<&str>) -> Program {
        let mut collected_cmds = String::new();
        match start_extern {
            true => {
                for cmd in cmds {
                    collected_cmds.push_str("Start-Process ");
                    collected_cmds.push_str(executer);
                    collected_cmds.push_str(" -ArgumentList '");
                    collected_cmds.push_str(cmd);
                    collected_cmds.push_str("'");
                    collected_cmds.push_str(";");
                }
            }
            false => {
                for cmd in cmds {
                    collected_cmds.push_str(executer);
                    collected_cmds.push_str(" ");
                    collected_cmds.push_str(cmd);
                    collected_cmds.push_str(";");
                }
            }
        }

        let name = name.to_string();

        Program {
            name,
            start_extern,
            collected_cmds,
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
    let scoop = Program::new("scoop", "scoop", true, vec!["update --all"]);
    let winget = Program::new("winget", "winget", true, vec!["upgrade"]);
    let rust = Program::new("rust", "rustup", true, vec!["update"]);
    let haskell = Program::new("haskel", "ghcup", true, vec!["update"]);
    let vim = Program::new("vim", "vim", true, vec!["-c PlugUpdate -c qa"]);
    let nvim = Program::new("nvim", "nvim", true, vec!["-c PlugUpdate -c qa"]);
    let pip = Program::new("pip", "py", true, vec!["-m pip install --upgrade pip"]);

    let commands: Vec<Program> = vec![scoop, winget, rust, haskell, vim, nvim, pip];

    // TESTS
    // let test1 = Program::new("sleep", Start-Sleep", false, vec![" -Seconds(2)"]);
    // let test2 = Program::new("sleep", Start-Sleep", false, vec![" -Seconds(3)"]);
    // let test3 = Program::new("sleep", Start-Sleep", false, vec![" -Seconds(4)"]);
    // let commands: Vec<Program> = vec![test1, test2, test3];

    if let Err(err) = update(commands) {
        error!("Error executing cmds: {}", err);
        process::exit(1);
    }
}

fn update(commands: Vec<Program>) -> Result<(), Box<dyn Error>> {
    println!("{}", "::: STARTING UPDATE".bold().yellow());

    let num = commands.len() as u64;
    progress_bar(commands, num)?;

    Ok(())
}

fn run_cmd(cmd: &str) -> Result<(), Box<dyn Error>> {
    if cfg!(target_os = "windows") {
        Command::new("powershell").args(["-c", cmd]).status()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo 'not implemented yet'")
            .status()?
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
        "{spinner:.green} [{elapsed_precise}] {bar:40.yellow/red} {pos:>5}/{len:5} {eta:5} {msg}",
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
                spinner.set_message(format!("{} {}", "updating".yellow(), arg.name));
                spinner.tick();
                run_cmd(arg.collected_cmds.as_str()).unwrap();
                spinner.finish_with_message(match arg.start_extern {
                    true => {
                        format!("{} {}", arg.name, "started".green())
                    }
                    false => {
                        format!("{} {}", arg.name, "done".bold().green())
                    }
                });
                pb.inc(1);
            })
        })
        .collect();

    for h in handles {
        let _ = h.join();
    }

    pb.finish_with_message(format!("{}", "done".bold().green()));

    // m.clear().unwrap();

    println!(
        "{} {}",
        "::: ALL STARTED IN".bold().green(),
        HumanDuration(started.elapsed()).to_string().bold().green()
    );

    Ok(m)
}
