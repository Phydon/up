use chrono::Local;
use log::warn;

use std::{env, fs};

pub struct Program {
    pub name: String,
    pub start_extern: bool,
    pub has_output: bool,
    pub outputfile: String,
    pub update_cmd: Option<String>,
    pub info_cmd: Option<String>,
}

impl Program {
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

    pub fn new(
        name: &str,
        executer: &str,
        start_extern: bool,
        has_output: bool,
        cmd_for_update: Option<&str>,
        cmd_for_info: Option<&str>,
    ) -> Program {
        let mut tmp = String::new();
        match env::temp_dir().as_os_str().to_str() {
            Some(dir) => {
                tmp.push_str(dir);
            }
            None => {
                let err_msg = "Can`t find temp directory";
                warn!("{err_msg}");
                // FIXME panics if dir already exists
                // TODO check dir.exists()
                let dir = "~/up_tmp/";
                fs::create_dir(dir).expect("Unable to find or create tmp dir");
                tmp.push_str(dir);
            }
        }
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

        let name = name.to_string();

        Program {
            name,
            start_extern,
            has_output,
            outputfile,
            update_cmd,
            info_cmd,
        }
    }
}
