use colored::*;

use std::{env, fs, io, path::Path};

pub fn check_create_dir() -> io::Result<String> {
    let mut tmp_path = env::temp_dir();
    tmp_path.push("up_tmp\\");

    if !tmp_path.as_path().exists() {
        fs::create_dir(&tmp_path)?;
    }

    let dir = tmp_path.into_os_string().into_string().unwrap();

    Ok(dir)
}

pub fn remove_tmps(tmp_dir_path: &str) -> io::Result<()> {
    for entry in fs::read_dir(tmp_dir_path)? {
        let entry = entry?;
        match entry.path().file_name() {
            Some(file) => {
                let filename = file.to_string_lossy();
                if filename.contains(&"up_output_".to_string()) {
                    fs::remove_file(entry.path())?;
                    println!("{} {:?}", "Removed:".red(), filename);
                }
            }
            None => {}
        }
    }

    Ok(())
}

pub fn show_log_file() -> io::Result<String> {
    let log_path = Path::new(&env::temp_dir()).join("up.log");
    match log_path.try_exists()? {
        true => {
            return Ok(format!(
                "{} {}\n{}",
                "Log location:".italic().dimmed(),
                &log_path.display(),
                fs::read_to_string(&log_path)?
            ));
        }
        false => {
            return Ok(format!(
                "{} {}",
                "No log file found:".red().bold().to_string(),
                log_path.display()
            ))
        }
    }
}

pub fn open_tmp(arg: &str) -> io::Result<()> {
    let mut tmp_path = env::temp_dir();
    tmp_path.push("up_tmp\\");
    for entry in fs::read_dir(tmp_path)? {
        let entry = entry?;
        match entry.path().file_name() {
            Some(file) => {
                let filename = file.to_string_lossy();
                match arg {
                    "all" => {
                        let content = fs::read_to_string(entry.path())?;
                        println!("{}:", filename.bold().yellow());
                        println!("{content}");
                    }
                    _ => {
                        if filename.contains(&arg.to_string()) {
                            let content = fs::read_to_string(entry.path())?;
                            println!("{}:", filename.bold().yellow());
                            println!("{content}");
                        }
                    }
                }
            }
            None => {}
        }
    }

    Ok(())
}
