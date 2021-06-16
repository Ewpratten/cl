use cli::get_cli_matches;
use colored::Colorize;
use commands::{
    book::{
        dump::exec_dump_book, edit::exec_edit_book, export::exec_export_book, new::exec_new_book,
    },
    log::new::exec_new_log,
    query::exec_query,
};

use crate::commands::log::edit::exec_edit_log;

mod cli;
mod commands;
mod lib;

fn main() {
    // Get CLI args
    let global_matches = get_cli_matches();

    // Handle calling the correct command
    if let Some(matches) = global_matches.subcommand_matches("book") {
        if let Some(sub_matches) = matches.subcommand_matches("new") {
            exec_new_book(
                sub_matches.value_of("name").unwrap(),
                sub_matches.value_of("grid"),
                sub_matches.value_of("callsign"),
                sub_matches.value_of("description"),
                sub_matches.is_present("default"),
                sub_matches.value_of("import"),
            );
        } else if let Some(sub_matches) = matches.subcommand_matches("edit") {
            exec_edit_book(
                sub_matches.value_of("name").unwrap(),
                sub_matches.value_of("grid"),
                sub_matches.value_of("callsign"),
                sub_matches.value_of("description"),
                sub_matches.is_present("default"),
            );
        } else if let Some(sub_matches) = matches.subcommand_matches("dump") {
            exec_dump_book(sub_matches.value_of("name").unwrap())
        } else if let Some(sub_matches) = matches.subcommand_matches("export") {
            exec_export_book(
                sub_matches.value_of("name").unwrap(),
                sub_matches.value_of("outfile").unwrap(),
            )
        }
    } else if let Some(matches) = global_matches.subcommand_matches("query") {
        exec_query(
            matches.value_of("callsign").unwrap(),
            matches.value_of("logbook"),
            matches.is_present("search_all"),
        )
    } else if let Some(matches) = global_matches.subcommand_matches("log") {
        if let Some(sub_matches) = matches.subcommand_matches("new") {
            exec_new_log(
                sub_matches.value_of("callsign").unwrap(),
                sub_matches
                    .value_of("frequency")
                    .unwrap()
                    .parse()
                    .expect(&format!(
                        "{} is not a valid number!",
                        "Frequency".bright_blue()
                    )),
                sub_matches.value_of("mode").unwrap(),
                sub_matches.value_of("logbook"),
                sub_matches.value_of("rst_sent"),
                sub_matches.value_of("rst_recv"),
                sub_matches.value_of("date"),
                sub_matches.value_of("time"),
                sub_matches.value_of("grid"),
                sub_matches.value_of("name"),
                sub_matches.value_of("notes"),
            )
        } else if let Some(sub_matches) = matches.subcommand_matches("edit") {
            exec_edit_log(
                sub_matches.value_of("id").unwrap(),
                sub_matches.value_of("callsign"),
                match sub_matches.value_of("frequency") {
                    Some(freq) => Some(freq.parse().expect(&format!(
                        "{} is not a valid number!",
                        "Frequency".bright_blue()
                    ))),
                    None => None,
                },
                sub_matches.value_of("mode"),
                sub_matches.value_of("rst_sent"),
                sub_matches.value_of("rst_recv"),
                sub_matches.value_of("date"),
                sub_matches.value_of("time"),
                sub_matches.value_of("grid"),
                sub_matches.value_of("name"),
                sub_matches.value_of("notes"),
            )
        }
    }
}
