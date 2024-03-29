use crate::programs::Program;

use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use owo_colors::colored::*;
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
// const F1: u8 = 255;
// const F2: u8 = 46;
// const F3: u8 = 95;
// const B1: u8 = 41;
// const B2: u8 = 0;
// const B3: u8 = 25;
// green
// const F4: u8 = 10;
// const F5: u8 = 255;
// const F6: u8 = 169;
const F4: u8 = 59;
const F5: u8 = 179;
const F6: u8 = 140;
// purple
// const F7: u8 = 127;
// const F8: u8 = 83;
// const F9: u8 = 191;
// const F7: u8 = 116;
// const F8: u8 = 58;
// const F9: u8 = 191;
// blue
const F10: u8 = 127;
const F11: u8 = 111;
const F12: u8 = 219;

pub fn init(commands: Vec<Program>, mode: &str) -> Result<(), Box<dyn Error>> {
    let num = commands.len() as u64;
    match mode {
        "update" => {
            println!(
                "{} {}",
                "↗",
                // "STARTING UPDATE".bold().truecolor(250, 0, 104)
                "STARTING UPDATE".bold()
            );
            progress_bar(commands, num, "update")?;
        }
        "info" => {
            println!(
                "{} {}",
                "🛈",
                // "GETTING INFORMATION".bold().truecolor(250, 0, 104)
                "GETTING INFORMATION".bold()
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
        "{spinner:.red} [{elapsed_precise}] {bar:40.white/white} {pos:>3}/{len:2} {percent}% {msg:>5}",
    )
    .unwrap()
    // .progress_chars("=>-");
    .progress_chars("=>:");

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
                arg.name.bold(),
                // arg.name.truecolor(127, 111, 219).dimmed(),
                arg.placeholder
            ));
            match mode {
                "update" => thread::spawn(move || {
                    spinner.set_message(format!("{}", "updating".truecolor(250, 0, 104),));
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
                            format!("{}", "done".truecolor(59, 179, 140))
                        }
                        false => {
                            format!(
                                "{}    \t|  {}",
                                "done".truecolor(59, 179, 140),
                                arg.msg.join(" "),
                            )
                        }
                    });
                    pb.inc(1);
                }),
                "info" => thread::spawn(move || {
                    spinner.set_message(format!("{}", "collecting info".truecolor(250, 0, 104),));
                    spinner.tick();
                    match arg.info_cmd {
                        Some(cmd) => {
                            run_cmd(cmd.as_str()).unwrap();
                        }
                        None => {
                            arg.msg.push("No information found".to_string());
                        }
                    }
                    spinner.finish_with_message(match arg.msg.is_empty() {
                        true => {
                            format!("{}", "done".truecolor(59, 179, 140))
                        }
                        false => {
                            format!(
                                "{}    \t|  {}",
                                "done".truecolor(59, 179, 140),
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

    pb.finish_with_message(format!("{}", "done".bold().truecolor(59, 179, 140)));

    // m.clear().unwrap();

    println!(
        "{} {} {}",
        "✔",
        "all done in".truecolor(59, 179, 140),
        HumanDuration(started.elapsed())
            .to_string()
            .truecolor(127, 111, 219)
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
        "{}             {}",
        "System name:".truecolor(F10, F11, F12),
        sys.name().unwrap().truecolor(F4, F5, F6).bold()
    );
    println!(
        "{}   {}",
        "System kernel version:".truecolor(F10, F11, F12),
        sys.kernel_version().unwrap().truecolor(F4, F5, F6).bold()
    );
    println!(
        "{}       {}",
        "System OS version:".truecolor(F10, F11, F12),
        sys.os_version().unwrap().truecolor(F4, F5, F6).bold()
    );
    println!(
        "{}        {}",
        "System host name:".truecolor(F10, F11, F12),
        sys.host_name().unwrap().truecolor(F4, F5, F6).bold()
    );

    // Number of CPUs:
    println!(
        "{}          {}",
        "Number of CPUs:".truecolor(F10, F11, F12),
        sys.cpus().len().to_string().truecolor(F4, F5, F6).bold()
    );
}

pub fn confirm(msg: &str) -> bool {
    loop {
        println!("{}", msg);

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
