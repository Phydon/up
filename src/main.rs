// hide console window on Windows in release
// #![windows_subsystem = "windows"]

use colored::*;
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use log::error;

use std::{
    error::Error,
    process::{self, Command},
    sync::Arc,
    time::Instant,
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
        "echo \"TEST1\";Start-sleep -Seconds(1)".to_string(),
        "echo \"TEST2\";Start-sleep -Seconds(1)".to_string(),
        "echo \"TEST3\";Start-sleep -Seconds(1)".to_string(),
        "echo \"TEST4\";Start-sleep -Seconds(1)".to_string(),
        "echo \"TEST5\";Start-sleep -Seconds(1)".to_string(),
        // "scoop update".to_string(),
        // "scoop status".to_string(),
        // "winget upgrade".to_string(),
        // "rustup update".to_string(),
        // "vim -c PlugUpdate -c qa".to_string(),
        // "nvim -c PlugUpdate -c qa".to_string(),
        // "ghcup upgrade".to_string(),
        // "Start-sleep -seconds 5".to_string(),
    ];

    if let Err(err) = run_cmd(commands) {
        error!("Error executing cmds: {}", err);
        process::exit(1);
    }
}

// fn run_cmd(commands: Vec<String>) -> Result<(), Box<dyn Error>> {
//     let args: String = collect_args(commands)?;
//     cmd(args.as_str())?;

//     Ok(())
// }

fn run_cmd(commands: Vec<String>) -> Result<(), Box<dyn Error>> {
    println!("{}", "::: STARTING UPDATE".bold().yellow());

    let num = commands.len() as u64;
    progress_bar(commands, num)?;

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

// fn collect_args(args_list: Vec<String>) -> Result<String, Box<dyn Error>> {
//     let mut combiner = String::new();
//     for arg in args_list {
//         combiner.push_str(&arg);
//         combiner.push_str("; ");
//     }

//     Ok(combiner)
// }

fn progress_bar(commands: Vec<String>, num: u64) -> Result<Arc<MultiProgress>, Box<dyn Error>> {
    let started = Instant::now();

    let m = Arc::new(MultiProgress::new());
    let sty = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>5}/{len:5} {eta:5} {msg}",
    )
    .unwrap()
    // .progress_chars("#>-");
    .progress_chars("##-");

    let pb = m.add(ProgressBar::new(num));
    pb.set_style(sty.clone());

    pb.tick();
    for arg in commands {
        cmd(arg.as_str())?;

        // let pb2 = m.add(ProgressBar::new(128));
        // pb2.set_style(sty.clone());
        // for _ in 0..128 {
        //     pb2.inc(1);
        //     thread::sleep(Duration::from_millis(5));
        // }
        // pb2.finish();

        pb.inc(1);
    }
    pb.finish_with_message(format!("{}", "done".bold().green()));

    println!(
        "{} {}",
        "::: DONE IN".bold().green(),
        HumanDuration(started.elapsed()).to_string().bold().green()
    );

    Ok(m)
}
