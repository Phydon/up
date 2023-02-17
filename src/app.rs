use clap::{arg, ArgAction, Command};

// TODO how to run only by typing in the name of the program (-> "up")
pub fn up() -> Command {
    Command::new("up")
        .about("Update programs, get status or system information.")
        .version("1.0.0")
        .author("Leann Phydon <leann.phydon@gmail.com")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("clean")
                .short_flag('c')
                .about("Remove all temporary files")
        )
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
                .about("List all available programs")
        )
        .subcommand(
            Command::new("log")
                .short_flag('L')
                .about("Show content of the log file")
        )
        .subcommand(
            Command::new("open")
                .short_flag('o')
                .about("Open the output files for the specified program")
                .arg(arg!(<PROGRAM> "The program for which the output should be displayed \nEnter \"all\" to open all available output files"))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("run")
                .short_flag('r')
                .about("Run updates")
                .arg(
                    arg!(-v --verbose "show output")
                        .action(ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("sys")
                .short_flag('s')
                .about("Show system information")
        )
}
