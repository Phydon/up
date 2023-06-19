use clap::{arg, ArgAction, Command};
use owo_colors::colored::*;

pub fn up() -> Command {
    Command::new("up")
        .bin_name("up")
        .before_help(format!(
            "{}\n{}",
            "UP".bold().truecolor(250, 0, 104),
            "Leann Phydon <leann.phydon@gmail.com>".italic().dimmed()
        ))
        .about("Update programs, get status or system information.")
        // TODO update version
        .version("1.0.3")
        .author("Leann Phydon <leann.phydon@gmail.com")
        .subcommand(
            Command::new("clean")
                .short_flag('c')
                .long_flag("clean")
                .about("Remove all temporary files")
        )
        .arg(arg!(-v --verbose "show output").action(ArgAction::SetTrue))
        // .subcommand(
        //     Command::new("exclude")
        //         .about("Exclude programs from update")
        //         .short_flag('e'),
            // .arg(arg!(<PROGRAM> "The programs to exclude from updates").num_args(1..))
            // .arg_required_else_help(true),
        // )
        .subcommand(
            Command::new("info")
                .about("Get status information (saved in output files)")
                .short_flag('i')
                .long_flag("info")
                .arg(
                    arg!(-v --verbose "show output")
                        .action(ArgAction::SetTrue)
                )
            // .arg(arg!(<PROGRAM> "The program to get information about"))
            // .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("list")
                .short_flag('l')
                .long_flag("list")
                .about("List all available programs")
        )
        .subcommand(
            Command::new("log")
                .short_flag('L')
                .long_flag("log")
                .about("Show content of the log file")
        )
        .subcommand(
            Command::new("open")
                .short_flag('o')
                .long_flag("open")
                .about("Open the output files for the specified program")
                .arg(arg!(<PROGRAM> "The program for which the output should be displayed \nEnter \"all\" to open all available output files"))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("sys")
                .short_flag('s')
                .long_flag("sys")
                .about("Show system information")
        )
}
