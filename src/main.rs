use cli::get_cli_matches;
use commands::book::{
    dump::exec_dump_book, edit::exec_edit_book, export::exec_export_book, new::exec_new_book,
};

mod cli;
mod commands;

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
    }
}
