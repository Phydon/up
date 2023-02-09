use std::{env, fs, io};

// fn main() {
//     TODO for cleaning command:
//     if let Ok(tmp_dir_work) = check_create_dir() {
//         if let Err(err) = remove_tmps(&tmp_dir_work) {
//             error!("Unable to remove tmp files: {err}");
//         }
//     }

//     TODO for cleaning command:
//     if let Ok(tmp_file) = open_tmp(tmp_path) {
//         println!("{tmp_file:?}");
//     }
// }

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
                let content = file.to_string_lossy();
                if content.contains(&"up_output_".to_string()) {
                    // TODO let user confirm before removing the files
                    fs::remove_file(entry.path())?;
                }
            }
            None => {}
        }
    }

    Ok(())
}

// fn open_tmp(tmp_filepath: &str) -> io::Result<()> {
//     todo!();
// }
