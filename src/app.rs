use clap::{arg, Command};

// TODO how to run only by typing in the name of the program ("uptest")
pub fn up() -> Command {
    Command::new("up")
        .about("Updates your stuff")
        .version("1.0.0")
        .author("Leann Phydon <leann.phydon@gmail.com")
        // .allow_missing_positional(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        // .allow_external_subcommands(true)
        .subcommand(
            Command::new("run")
                .short_flag('r')
                // .long_flag("run")
                .about("run updates"),
        )
        .subcommand(
            Command::new("clean")
                .short_flag('c')
                .about("remove all temporary files"),
        )
        .subcommand(
            Command::new("open")
                .short_flag('o')
                .about("open the specified output files")
                .arg(arg!(<PROGRAM> "Open the output file of the specified program"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("info")
                .about("print status informations")
                .short_flag('i')
                .arg(arg!(<PROGRAM> "The program to get information about"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("exclude")
                .about("exclude programs from update")
                .short_flag('e')
                .arg(arg!(<PROGRAM> "The programs to exclude from updates").num_args(1..))
                .arg_required_else_help(true),
        )
}
