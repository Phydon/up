use chrono::Local;
use std::fmt;

const PLACEHOLDER_THRESHOLD: usize = 8;

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
        name: &str,
        symbol: Option<&str>,
        executer: &str,
        start_extern: bool,
        has_output: bool,
        cmd_for_update: Option<&str>,
        cmd_for_info: Option<&str>,
        tmp_dir: &str,
    ) -> Program {
        let mut tmp = tmp_dir.to_string();
        tmp.push_str("up_output_");
        tmp.push_str(name);
        let outputfile = tmp.to_string();

        let update_cmd = Self::collect_cmds(
            executer,
            start_extern,
            has_output,
            cmd_for_update,
            &outputfile,
        );
        let info_cmd = Self::collect_cmds(
            executer,
            start_extern,
            has_output,
            cmd_for_info,
            &outputfile,
        );

        let msg = Vec::new();
        let placeholder = Self::get_placeholder(name);

        let mut symbol_str = String::new();
        match symbol {
            Some(sym) => {
                symbol_str.push_str(sym);
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
        executer: &str,
        start_extern: bool,
        has_output: bool,
        cmd: Option<&str>,
        outputfile: &str,
    ) -> Option<String> {
        let datetime = Local::now().format("%d%m%Y_%H%M%S_%f").to_string();
        let mut output = String::new();
        output.push_str(outputfile);
        output.push_str("_");
        output.push_str(datetime.as_str());
        output.push_str(".txt");

        let mut collected_cmds = String::new();
        match cmd {
            Some(cmd) => match start_extern {
                true => {
                    collected_cmds.push_str("Start-Process ");
                    collected_cmds.push_str(executer);
                    collected_cmds.push_str(" -ArgumentList '");
                    collected_cmds.push_str(cmd);
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
                    collected_cmds.push_str(executer);
                    collected_cmds.push_str(" ");
                    collected_cmds.push_str(cmd);
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
