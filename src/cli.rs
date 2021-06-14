use clap::{App, AppSettings, Arg, ArgMatches, SubCommand, crate_authors, crate_description, crate_name, value_t};

pub fn get_cli_matches<'a>() -> ArgMatches<'a> {
    App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("book")
                .alias("b")
                .help("Tools for managing logbooks")
                .subcommand(
                    SubCommand::with_name("new").alias("n")
                        .help("Create a new logbook")
                        .arg(
                            Arg::with_name("name")
                                .help("Logbook name")
                                .takes_value(true)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("grid")
                                .long("grid")
                                .short("g")
                                .help("Optional gridsquare for this logbook (useful for field logging)")
                                .takes_value(true)
                                .required(false),
                        )
                        .arg(
                            Arg::with_name("callsign")
                                .long("callsign")
                                .short("c")
                                .help("Callsign attached to this logbook")
                                .takes_value(true)
                                .required(false),
                        )
                        .arg(
                            Arg::with_name("description")
                                .long("description")
                                .short("d")
                                .help("Optional description for this logbook")
                                .takes_value(true)
                                .required(false),
                        )
                        .arg(
                            Arg::with_name("default")
                            .long("make-default")
                            .help("Make this the default logbook for this device")
                            .takes_value(false)
                            .required(false),
                        )
                        .arg(
                            Arg::with_name("import")
                            .long("import")
                            .help("Import logs from an ADIF file into a new logbook")
                            .takes_value(true)
                            .required(false),
                        ),
                )
                .subcommand(SubCommand::with_name("edit").alias("e")
                    .help("Edit an existing logbook")
                    .arg(
                        Arg::with_name("name")
                            .help("Logbook name")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("grid")
                            .long("grid")
                            .short("g")
                            .help("Optional gridsquare for this logbook (useful for field logging)")
                            .takes_value(true)
                            .required(false),
                    )
                    .arg(
                        Arg::with_name("callsign")
                            .long("callsign")
                            .short("c")
                            .help("Callsign attached to this logbook")
                            .takes_value(true)
                            .required(false),
                    )
                    .arg(
                        Arg::with_name("description")
                            .long("description")
                            .short("d")
                            .help("Optional description for this logbook")
                            .takes_value(true)
                            .required(false),
                    )
                    .arg(
                        Arg::with_name("default")
                            .long("make-default")
                            .help("Make this the default logbook for this device")
                            .takes_value(false)
                            .required(false),
                    ),
                )
                .subcommand(SubCommand::with_name("dump").alias("d")
                    .help("Dump the contents of a logbook")
                    .arg(
                        Arg::with_name("name")
                            .help("Logbook name")
                            .takes_value(true)
                            .required(true),
                    )
                )
                .subcommand(SubCommand::with_name("export")
                    .help("Export the contents of a logbook to a file")
                    .arg(
                        Arg::with_name("name")
                            .help("Logbook name")
                            .takes_value(true)
                            .required(true),
                    )
                    .arg(
                        Arg::with_name("outfile")
                            .help("Output file path")
                            .takes_value(true)
                            .required(true)
                    )
                )
        )
        .subcommand(
            SubCommand::with_name("query")
            .alias("q")
            .help("Look up existing logbook entries")
            .arg(
                Arg::with_name("search_all")
                    .long("search-all")
                    .short("a")
                    .help("Search all logbooks")
                    .takes_value(false)
                    .required(false)
            )
            .arg(
                Arg::with_name("callsign")
                    .long("callsign")
                    .short("c")
                    .help("Search by callsign")
                    .takes_value(true)
                    .required(false)
            )
        )
        .subcommand(
            SubCommand::with_name("log")
                .alias("l")
                .help("Manage log entries")
                .subcommand(
                    SubCommand::with_name("new")
                        .alias("n")   
                        .help("Create a new log entry")
                        .arg(
                            Arg::with_name("logbook")
                                .long("logbook")
                                .short("l")
                                .help("Use a specific logbook")
                                .takes_value(true)
                                .required(false)
                        )
                        .arg(
                            Arg::with_name("callsign")
                                .help("Callsign of the station you contacted")
                                .takes_value(true)
                                .required(true)
                        )
                        .arg(
                            Arg::with_name("frequency")
                                .long("frequency")
                                .short("f")
                                .help("QSO frequency in KHz (ex. 7030)")
                                .takes_value(true)
                                .required(true)
                        )
                        .arg(
                            Arg::with_name("mode")
                                .long("mode")
                                .short("m")
                                .help("Operating mode")
                                .takes_value(true)
                                .required(true)
                        )
                        .arg(
                            Arg::with_name("time")
                                .long("override-time")
                                .short("t")
                                .help("Specify a custom time for the log entry. Assumes local time unless the string ends with \"z\". (ex. 14:04 or 20:04z)")
                                .takes_value(true)
                                .required(false)
                        )
                        .arg(
                            Arg::with_name("grid")
                                .long("gridsquare")
                                .short("g")
                                .help("The station's gridsquare")
                                .required(false)
                                .takes_value(true)
                        )
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .short("n")
                                .help("Name of the station operator")
                                .takes_value(true)
                                .required(false)
                        )
                        .arg(
                            Arg::with_name("notes")
                                .long("notes")
                                .help("Any QSO notes")
                                .takes_value(true)
                                .required(false)
                        )
                )
                .subcommand(
                    SubCommand::with_name("edit")
                        .alias("e")   
                        .help("Edit a log entry")
                        .arg(
                            Arg::with_name("id")
                                .help("Log entry ID to edit")
                                .takes_value(true)
                                .required(true)
                        )
                        .arg(
                            Arg::with_name("callsign")
                                .help("Callsign of the station you contacted")
                                .takes_value(true)
                                .required(true)
                        )
                        .arg(
                            Arg::with_name("frequency")
                                .long("frequency")
                                .short("f")
                                .help("QSO frequency in KHz (ex. 7030)")
                                .takes_value(true)
                                .required(true)
                        )
                        .arg(
                            Arg::with_name("mode")
                                .long("mode")
                                .short("m")
                                .help("Operating mode")
                                .takes_value(true)
                                .required(true)
                        )
                        .arg(
                            Arg::with_name("time")
                                .long("override-time")
                                .short("t")
                                .help("Specify a custom time for the log entry. Assumes local time unless the string ends with \"z\". (ex. 14:04 or 20:04z)")
                                .takes_value(true)
                                .required(false)
                        )
                        .arg(
                            Arg::with_name("grid")
                                .long("gridsquare")
                                .short("g")
                                .help("The station's gridsquare")
                                .required(false)
                                .takes_value(true)
                        )
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .short("n")
                                .help("Name of the station operator")
                                .takes_value(true)
                                .required(false)
                        )
                        .arg(
                            Arg::with_name("notes")
                                .long("notes")
                                .help("Any QSO notes")
                                .takes_value(true)
                                .required(false)
                        )
                )
        )
        .get_matches()
}
