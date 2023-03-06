# up

**Update programs, get status or system information**

Command line tool to update several programs at the same time
* reads in programs from a config file
* creates a default config file if no config file exists
* Update programs from that config file
* Get status information about the programs
* List all included programs
* Show the output of the last update or status request
* Remove all stored output
* Get quick system information

![screenshot](https://github.com/Phydon/up/blob/master/assets/screenshot_starting_update.png)

![screenshot](https://github.com/Phydon/up/blob/master/assets/screenshot_updating.png)

![screenshot](https://github.com/Phydon/up/blob/master/assets/screenshot_update_done.png)

## Usage

* run ```up``` to update all programs


```
up <COMMAND>

Commands:
  clean, -c  Remove all temporary files
  info, -i   Get status information (saved in output files)
  list, -l   List all available programs
  log, -L    Show content of the log file
  open, -o   Open the output files for the specified program
  sys, -s    Show system information
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Installation

via Cargo or get the ![binary](https://github.com/Phydon/up/releases)

## TODO

* exclude programs
* add new programs via command line (e.g. "up add") to config file
* colored output as an optional flag or make different colors available

