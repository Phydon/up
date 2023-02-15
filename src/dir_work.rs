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
    match Path::new("up.log").try_exists()? {
        true => {
            return Ok(fs::read_to_string("up.log")?);
        }
        false => return Ok("No log file found".red().bold().to_string()),
    }
}

pub fn open_tmp(program_name: &str) -> io::Result<()> {
    let mut tmp_path = env::temp_dir();
    tmp_path.push("up_tmp\\");
    for entry in fs::read_dir(tmp_path)? {
        let entry = entry?;
        match entry.path().file_name() {
            Some(file) => {
                let filename = file.to_string_lossy();
                if filename.contains(&program_name.to_string()) {
                    let content = fs::read_to_string(entry.path())?;
                    println!("{}:", filename.bold().yellow());
                    println!("{content}");
                }
            }
            None => {}
        }
    }

    Ok(())
}
