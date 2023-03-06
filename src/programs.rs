use chrono::Local;
use log::error;
use serde::Deserialize;

use std::{
    fmt,
    fs::{self, File},
    io,
    path::PathBuf,
    process,
};

use crate::dir_work::check_create_tmp_dir;

const PLACEHOLDER_THRESHOLD: usize = 8;

#[derive(Clone, Deserialize)]
struct Config {
    apps: Vec<App>,
}

#[derive(Clone, Deserialize)]
struct App {
    name: String,
    symbol: Option<String>,
    executer: String,
    start_extern: bool,
    has_output: bool,
    cmd_for_update: Option<String>,
    cmd_for_info: Option<String>,
}

#[derive(Clone)]
pub struct Program {
    pub name: String,
    pub symbol: String,
    pub start_extern: bool,
    pub has_output: bool,
    pub outputfile: String,
    pub update_cmd: Option<String>,
    pub info_cmd: Option<String>,
    pub msg: Vec<String>,
    pub placeholder: String,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Program {
    pub fn new(
        name: String,
        symbol: Option<String>,
        executer: String,
        start_extern: bool,
        has_output: bool,
        cmd_for_update: Option<String>,
        cmd_for_info: Option<String>,
    ) -> Program {
        let mut tmp = check_create_tmp_dir().unwrap_or_else(|err| {
            error!("Unable to find or create a temporary directory: {err}");
            process::exit(1);
        });
        tmp.push_str("up_output_");
        tmp.push_str(&name);
        let outputfile = tmp.to_string();

        let update_cmd = Self::collect_cmds(
            &executer,
            start_extern,
            has_output,
            cmd_for_update,
            &outputfile,
        );
        let info_cmd = Self::collect_cmds(
            &executer,
            start_extern,
            has_output,
            cmd_for_info,
            &outputfile,
        );

        let msg = Vec::new();
        let placeholder = Self::get_placeholder(&name);

        let mut symbol_str = String::new();
        match symbol {
            Some(sym) => {
                symbol_str.push_str(&sym);
            }
            None => {
                symbol_str.push_str(
                    name.chars()
                        .nth(0)
                        .expect("Unable to extract the first char from progam name")
                        .to_ascii_uppercase()
                        .to_string()
                        .as_str(),
                );
            }
        }
        let symbol = symbol_str;

        let name = name.to_string();

        Program {
            name,
            symbol,
            start_extern,
            has_output,
            outputfile,
            update_cmd,
            info_cmd,
            msg,
            placeholder,
        }
    }

    fn collect_cmds(
        executer: &String,
        start_extern: bool,
        has_output: bool,
        cmd: Option<String>,
        outputfile: &String,
    ) -> Option<String> {
        let datetime = Local::now().format("%d%m%Y_%H%M%S_%f").to_string();
        let mut output = String::new();
        output.push_str(&outputfile);
        output.push_str("_");
        output.push_str(datetime.as_str());
        output.push_str(".txt");

        let mut collected_cmds = String::new();
        match cmd {
            Some(cmd) => match start_extern {
                true => {
                    collected_cmds.push_str("Start-Process ");
                    collected_cmds.push_str(&executer);
                    collected_cmds.push_str(" -ArgumentList '");
                    collected_cmds.push_str(&cmd);
                    collected_cmds.push_str("'");
                    if has_output {
                        collected_cmds.push_str("-RedirectStandardOutput ");
                        collected_cmds.push_str(output.as_str());
                    }
                    collected_cmds.push_str(" -WindowStyle Hidden");
                    collected_cmds.push_str(" -Wait");
                    collected_cmds.push_str(";");
                }
                false => {
                    collected_cmds.push_str(&executer);
                    collected_cmds.push_str(" ");
                    collected_cmds.push_str(&cmd);
                    collected_cmds.push_str(";");
                }
            },
            None => return None,
        }

        Some(collected_cmds)
    }

    fn get_placeholder(name: &str) -> String {
        let mut holder = String::new();
        let rest_length = PLACEHOLDER_THRESHOLD - name.len();
        for _ in 0..rest_length {
            holder.push_str(" ");
        }

        holder
    }
}

pub fn load_programs(path: &PathBuf) -> io::Result<Vec<Program>> {
    if !path.as_path().exists() {
        let default_content = format!(
            "// {}\n// {}\n// {}\n// {}\n// {}\n// {}\n// {}\n// {}\n// {}\n// {}\n// {}\n// {}\n// {}\n// {}\n// {}\n// {}\n// {}\n// {}\n{}",
            "Usage:\n",
            "App(",
            "\tname: \"example\",",
            "\tsymbol: None,",
            "\texecuter: \"example\",",
            "\tstart_extern: true,",
            "\thas_output: true,",
            "\tcmd_for_update: Some(\"-c example update --all\"),",
            "\tcmd_for_info: Some(\"-c example status\"),",
            "),\n",
            "Values to replace:",
            "<name>           => a custom name for the program",
            "<symbol>         => if symbol is \"None\", the first character of the name will be used; options: [Some(\"<symbol>\"), None]",
            "<excuter>        => the actual program to call from the command line (often the name of the program itself)",
            "<start_extern>   => should only be \"false\" if no output will be produced and no external program starts; options [true, false]",
            "<has_output>     => used to write the output in a temporary file for later reference; options [true, false]",
            "<cmd_for_update> => the actual command to update the program; options: [Some(\"<cmd_for_update>\"), None]",
            "<cmd_for_info>   => the actual command to get status information about the program; options: [Some(\"<cmd_for_info>\"), None]\n",
            "(\n \tapps: [\n \t\tApp(\n \t\t\tname: \"example\",\n \t\t\tsymbol: None,\n \t\t\texecuter: \"example\",\n \t\t\tstart_extern: true,\n \t\t\thas_output: true,\n \t\t\tcmd_for_update: None,\n \t\t\tcmd_for_info: None,\n \t\t),\n \t],\n)"
        );
        fs::write(&path, default_content)?;
    }

    let file = File::open(path)?;
    let config: Config = ron::de::from_reader(file).unwrap_or_else(|err| {
        error!("Unable to read ron file {}: {}", path.display(), err);
        process::exit(1);
    });

    let mut programs = Vec::new();
    for app in config.apps {
        let program = Program::new(
            app.name,
            app.symbol,
            app.executer,
            app.start_extern,
            app.has_output,
            app.cmd_for_update,
            app.cmd_for_info,
        );
        programs.push(program);
    }

    Ok(programs)
}
