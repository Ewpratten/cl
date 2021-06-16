use cli_table::{ColorChoice, Style, Table, format::{Border, HorizontalLine}};
use colored::Colorize;

use crate::lib::{data::logbook::Logbook, ensure_storage_location_exists, get_data_books_path};

pub fn exec_dump_book(name: &str) {
    // Ensure we have our directories
    ensure_storage_location_exists();

    // Check if the book exists
    let mut book_path = get_data_books_path();
    book_path.push(format!("{}.json", name));
    if !std::fs::metadata(&book_path).is_ok() {
        println!("{}", "Logbook does not exist!".bright_red());
        return;
    }

    // Load the book
    let book: Logbook = autojson::structify(&book_path).expect("Could not deserialize logbook");

    // Build the table
    let table = book
        .entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            vec![
                format!("{}", index),
                entry.callsign.clone().bright_blue().to_string(),
                entry.band.clone(),
                entry.mode.clone(),
                entry.sent_rst.clone().unwrap_or("???".to_string()),
                entry.recv_rst.clone().unwrap_or("???".to_string()),
                entry.time.date().to_string(),
            ]
        })
        .collect::<Vec<Vec<String>>>()
        .table()
        .title(vec![
            "Index".green().bold(),
            "Callsign".green().bold(),
            "Band".green().bold(),
            "Mode".green().bold(),
            "RST>".green().bold(),
            ">RST".green().bold(),
            "Date".green().bold(),
        ]);

    // Print the table
    cli_table::print_stdout(table).unwrap();
}
