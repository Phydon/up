# up

**Update programs, get status or system information**

Command line tool to update several programs at the same time
* Update programs
* Get status information about the programs
* List all included programs
* Show the output of the last update or status request
* Remove all stored output
* Get quick system information

![screenshot](https://github.com/Phydon/up/blob/master/assets/screenshot_starting_update.png)

![screenshot](https://github.com/Phydon/up/blob/master/assets/screenshot_updating.png)

![screenshot](https://github.com/Phydon/up/blob/master/assets/screenshot_update_done.png)

## Usage

```
up <COMMAND>

Commands:
  clean, -c  Remove all temporary files
  info, -i   Get status information (saved in output files)
  list, -l   List all available programs
  log, -L    Show content of the log file
  open, -o   Open the output files for the specified program
  run, -r    Run updates
  sys, -s    Show system information
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## TODO

* exclude programs
* read in programs to update from a config file
* add new programs via command line (e.g. "up add") to this config file
* colored output as an optional flag or make different colors available

