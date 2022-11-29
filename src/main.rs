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
    thread,
    time::{Duration, Instant},
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
        // "echo \"START1\";Start-sleep -Seconds(2);echo \"END1\"".to_string(),
        // "echo \"START2\";Start-sleep -Seconds(3);echo \"END2\"".to_string(),
        // "echo \"START3\";Start-sleep -Seconds(2);echo \"END3\"".to_string(),
        // "echo \"START4\";Start-sleep -Seconds(1);echo \"END4\"".to_string(),
        // "echo \"START5\";Start-sleep -Seconds(4);echo \"END5\"".to_string(),
        "Start-sleep -seconds 4".to_string(),
        "Start-sleep -seconds 2".to_string(),
        "Start-sleep -seconds 2".to_string(),
        "Start-sleep -seconds 1".to_string(),
        "Start-sleep -seconds 3".to_string(),
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

// fn progress_bar(commands: Vec<String>, num: u64) -> Result<Arc<MultiProgress>, Box<dyn Error>> {
//     let started = Instant::now();

//     let m = Arc::new(MultiProgress::new());
//     let sty = ProgressStyle::with_template(
//         "{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>5}/{len:5} {eta:5} {msg}",
//     )
//     .unwrap()
//     .progress_chars("#>-");

//     let pb = m.add(ProgressBar::new(num));
//     pb.set_style(sty.clone());

//     pb.tick();
//     for arg in commands {
//         cmd(arg.as_str())?;

//         // let pb2 = m.add(ProgressBar::new(128));
//         // pb2.set_style(sty.clone());
//         // for _ in 0..128 {
//         //     pb2.inc(1);
//         //     thread::sleep(Duration::from_millis(5));
//         // }
//         // pb2.finish();

//         pb.inc(1);
//     }
//     pb.finish_with_message(format!("{}", "done".bold().green()));

//     println!(
//         "{} {}",
//         "::: DONE IN".bold().green(),
//         HumanDuration(started.elapsed()).to_string().bold().green()
//     );

//     Ok(m)
// }

fn progress_bar(commands: Vec<String>, num: u64) -> Result<Arc<MultiProgress>, Box<dyn Error>> {
    let started = Instant::now();
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    let m = Arc::new(MultiProgress::new());
    let sty = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>5}/{len:5} {eta:5} {msg}",
    )
    .unwrap()
    .progress_chars("#>-");

    let pb = m.add(ProgressBar::new(num));
    // TODO remove clone?
    pb.set_style(sty.clone());

    pb.tick();
    // let handles: Vec<_> = (0..num)
    let handles: Vec<_> = commands
        .into_iter()
        .map(|arg| {
            // // FIXME
            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(Duration::from_millis(200));
            pb.set_style(spinner_style.clone());
            // let pb = m.add(ProgressBar::new(num));
            // pb.set_style(spinner_style.clone());
            pb.set_prefix(format!("[..]"));
            thread::spawn(move || {
                pb.set_message(format!("{}", "updating".red()));
                // pb.inc(1);
                pb.tick();
                cmd(arg.as_str()).unwrap();
                pb.finish_with_message(format!("{}", "done".bold().green()));
                // pb.finish();
            })
        })
        .collect();

    for h in handles {
        let _ = h.join();
    }

    pb.finish_with_message(format!("{}", "done".bold().green()));

    m.clear().unwrap();

    println!(
        "{} {}",
        "::: DONE IN".bold().green(),
        HumanDuration(started.elapsed()).to_string().bold().green()
    );

    Ok(m)
}
