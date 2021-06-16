# CL
[![Crates.io](https://img.shields.io/crates/v/cllog)](https://crates.io/crates/cllog) 
[![Build](https://github.com/Ewpratten/cl/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/cl/actions/workflows/build.yml)

`cl` is my command-line amateur radio logging tool.

## Installation

`cl` can be installed via `cargo`:

```sh
cargo install cl
```

## Usage

`cl` is built up of sub-commands:

```
# $ cl --help
Evan Pratten <ewpratten@gmail.com>
VA3ZZA's CL amateur radio logging tool

USAGE:
    cl [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    book     Tools for managing logbooks
    help     Prints this message or the help of the given subcommand(s)
    log      Manage log entries
    query    Look up existing logbook entries
```

### Import and export

`cl` can both import and export ADIF files. This is useful for data migration, and exporting to LOTW.

### Common commands

All commands will print their help message if run without arguments.

#### `cl log new`

Used to add a new entry to the logbook. Help info:

```
Create a new log entry

USAGE:
    cl log new [OPTIONS] <callsign> --frequency <frequency> --mode <mode>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --override-date <date>       Specify a custom date for the log entry. Format yyyy-mm-dd
    -f, --frequency <frequency>      QSO frequency in KHz (ex. 7030)
    -g, --gridsquare <grid>          The station's gridsquare
    -l, --logbook <logbook>          Use a specific logbook
    -m, --mode <mode>                Operating mode
    -n, --name <name>                Name of the station operator
        --notes <notes>              Any QSO notes
    -r, --rst-received <rst_recv>    RST received from the other station
    -s, --rst-sent <rst_sent>        RST sent to the other station
    -t, --override-time <time>       Specify a custom time for the log entry
        --tx-pwr <tx_pwr>            Number of watts used for this QSO

ARGS:
    <callsign>    Callsign of the station you contacted
```

#### `cl query`

Used for looking up existing entries in a logbook. The callsign field excepts REGEX strings for easy searching. Help info:

```
Look up existing logbook entries

USAGE:
    cl query [FLAGS] --callsign <callsign> [logbook]

FLAGS:
    -h, --help          Prints help information
    -a, --search-all    Search all logbooks
    -V, --version       Prints version information

OPTIONS:
    -c, --callsign <callsign>    Search by callsign

ARGS:
    <logbook>    Logbook to search (otherwise default or all)
```