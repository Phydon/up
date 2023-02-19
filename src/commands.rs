use crate::programs::Program;

use colored::*;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use sysinfo::{CpuRefreshKind, RefreshKind, System, SystemExt};

use std::{
    error::Error,
    io,
    process::Command,
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

pub fn init(commands: Vec<Program>, mode: &str) -> Result<(), Box<dyn Error>> {
    let num = commands.len() as u64;
    match mode {
        "update" => {
            println!("{} {}", "↗", "STARTING UPDATE".bold().truecolor(F7, F8, F9));
            progress_bar(commands, num, "update")?;
        }
        "info" => {
            println!(
                "{} {}",
                "🛈",
                "GETTING INFORMATION".bold().truecolor(F7, F8, F9)
            );
            progress_bar(commands, num, "info")?;
        }
        _ => {
            unreachable!();
        }
    }

    Ok(())
}

fn run_cmd(cmd: &str) -> Result<(), Box<dyn Error>> {
    if cfg!(target_os = "windows") {
        Command::new("powershell").args(["-c", cmd]).status()?
    } else {
        unimplemented!();
    };

    Ok(())
}

fn progress_bar(
    commands: Vec<Program>,
    num: u64,
    mode: &str,
) -> Result<Arc<MultiProgress>, Box<dyn Error>> {
    let started = Instant::now();
    let spinner_style = ProgressStyle::with_template("{prefix} {spinner:.red} {wide_msg}").unwrap();
    // .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    let m = Arc::new(MultiProgress::new());
    let sty = ProgressStyle::with_template(
        "{spinner:.red} [{elapsed_precise}] {bar:40.red/white} {pos:>5}/{len:5} {eta:5} {msg}",
    )
    .unwrap()
    // .progress_chars("#>-");
    .progress_chars("=>-");

    let pb = m.add(ProgressBar::new(num));
    pb.set_style(sty);

    pb.tick();
    let handles: Vec<_> = commands
        .into_iter()
        .map(|mut arg| {
            let pb = pb.clone();
            let spinner = m.add(ProgressBar::new_spinner());
            spinner.enable_steady_tick(Duration::from_millis(200));
            spinner.set_style(spinner_style.clone());
            spinner.set_prefix(format!(
                "[ {} ] {}{}",
                arg.symbol.dimmed(),
                arg.name.truecolor(F7, F8, F9),
                arg.placeholder
            ));
            match mode {
                "update" => thread::spawn(move || {
                    spinner.set_message(format!("{}", "updating".truecolor(F7, F8, F9),));
                    spinner.tick();
                    match arg.update_cmd {
                        Some(cmd) => {
                            run_cmd(cmd.as_str()).unwrap();
                        }
                        None => {
                            arg.msg.push("No update command found".to_string());
                        }
                    }
                    spinner.finish_with_message(match arg.msg.is_empty() {
                        true => {
                            format!("{}", "done".truecolor(F4, F5, F6))
                        }
                        false => {
                            format!(
                                "{}    \t|  {}",
                                "done".truecolor(F4, F5, F6),
                                arg.msg.join(" "),
                            )
                        }
                    });
                    pb.inc(1);
                }),
                "info" => thread::spawn(move || {
                    spinner.set_message(format!("{}", "collecting info".truecolor(F7, F8, F9),));
                    spinner.tick();
                    match arg.info_cmd {
                        Some(cmd) => {
                            run_cmd(cmd.as_str()).unwrap();
                        }
                        None => {
                            arg.msg.push("No information command found".to_string());
                        }
                    }
                    spinner.finish_with_message(match arg.msg.is_empty() {
                        true => {
                            format!("{}", "done".truecolor(F4, F5, F6))
                        }
                        false => {
                            format!(
                                "{}    \t|  {}",
                                "done".truecolor(F4, F5, F6),
                                arg.msg.join(" "),
                            )
                        }
                    });
                    pb.inc(1);
                }),
                _ => {
                    unreachable!()
                }
            }
        })
        .collect();

    for h in handles {
        let _ = h.join();
    }

    pb.finish_with_message(format!("{}", "done".bold().truecolor(F4, F5, F6)));

    // m.clear().unwrap();

    println!(
        "{} {} {}",
        "✔",
        "ALL DONE IN ".bold().truecolor(F4, F5, F6),
        HumanDuration(started.elapsed())
            .to_string()
            .to_uppercase()
            .bold()
            .truecolor(F1, F2, F3)
            .on_truecolor(B1, B2, B3)
    );

    Ok(m)
}

pub fn get_sys() {
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_users_list(),
    );

    // Display system information:
    println!(
        "{}:             {}",
        "System name".truecolor(F7, F8, F9),
        sys.name().unwrap().truecolor(F4, F5, F6).bold()
    );
    println!(
        "{}:   {}",
        "System kernel version".truecolor(F7, F8, F9),
        sys.kernel_version().unwrap().truecolor(F4, F5, F6).bold()
    );
    println!(
        "{}       {}",
        "System OS version:".truecolor(F7, F8, F9),
        sys.os_version().unwrap().truecolor(F4, F5, F6).bold()
    );
    println!(
        "{}        {}",
        "System host name:".truecolor(F7, F8, F9),
        sys.host_name().unwrap().truecolor(F4, F5, F6).bold()
    );

    // Number of CPUs:
    println!(
        "{}          {}",
        "Number of CPUs:".truecolor(F7, F8, F9),
        sys.cpus().len().to_string().truecolor(F4, F5, F6).bold()
    );
}

pub fn confirm(msg: &str) -> bool {
    loop {
        println!("{}", msg.yellow().bold());

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => {}
        }
    }
}

pub fn list_programs(programs: &Vec<Program>) {
    println!("{}", "Available programs:".bold().yellow());
    for program in programs {
        println!(
            "[ {} ] {}",
            program.symbol.dimmed(),
            program.name.truecolor(F4, F5, F6).bold()
        );
    }
}

// FIXME
// pub fn exclude(programs: &Vec<Program>) -> io::Result<Vec<Program>> {
//     let mut filtered = Vec::new();
//     Ok(filtered)
// }
