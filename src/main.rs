use clap::{crate_authors, crate_description, crate_name, value_t, App, Arg, SubCommand};

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("book")
                .alias("b")
                .help("Tools for managing logbooks")
                .subcommand(
                    SubCommand::with_name("new")
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
                        ),
                )
                .subcommand(SubCommand::with_name("edit")
                    .help("Edit an existing logbook")
                    .arg(
                        Arg::with_name("name")
                        .help("Logbook name")
                        .takes_value(true)
                        .required(true),
                    )
                )
        )
        .get_matches();
}
