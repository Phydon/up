use crate::programs::Program;

use colored::*;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};

use std::{
    error::Error,
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

pub fn update(commands: Vec<Program>) -> Result<(), Box<dyn Error>> {
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
