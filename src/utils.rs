use std::{env, fs, io};

// TODO for cleaning command:
// fn main() {
//     if let Ok(tmp_dir_work) = check_create_dir() {
//         if let Err(err) = remove_tmps(&tmp_dir_work) {
//             error!("Unable to remove tmp files: {err}");
//         }
//     }
// }

fn check_create_dir() -> io::Result<String> {
    let mut tmp_path = env::temp_dir();
    tmp_path.push("up_tmp/");

    if !tmp_path.as_path().exists() {
        fs::create_dir(&tmp_path)?;
    }

    let dir = tmp_path.into_os_string().into_string();

    Ok(dir)
}

fn remove_tmps(tmp_dir_work_path: &str) -> io::Result<()> {
    for entry in fs::read_dir(tmp_dir_work_path)? {
        let entry = entry?;
        match entry.path().file_name() {
            Some(file) => {
                let content = file.to_string_lossy();
                if content.contains(&"up_output_".to_string()) {
                    //TODO
                    // fs::remove_file(entry.path())?;
                    println!("Removing tmp files");
                }
            }
            None => {}
        }
    }

    Ok(())
}
