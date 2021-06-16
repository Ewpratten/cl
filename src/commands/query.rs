use cli_table::Table;
use colored::Colorize;
use regex::Regex;

use crate::lib::{
    data::logbook::{book_name_or_default, Logbook},
    ensure_storage_location_exists, get_data_books_path, get_data_default_book_path,
};

pub fn exec_query(callsign: &str, logbook: Option<&str>, search_all: bool) {
    // Ensure we have our directories
    ensure_storage_location_exists();

    // Build a list of books to search
    let mut book_list = Vec::new();
    if !search_all {
        book_list.push(book_name_or_default(logbook));
    } else {
        for file in std::fs::read_dir(get_data_books_path()).expect("Could not list all logbooks") {
            book_list.push(
                file.unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .split('.')
                    .collect::<Vec<&str>>()
                    .iter()
                    .next()
                    .unwrap()
                    .to_string(),
            );
        }
    }

    // Build a regex searcher for the callsign
    let callsign_re = Regex::new(callsign.to_uppercase().as_str()).unwrap();

    // Load every book to search their entries
    for book_name in book_list {
        // Check if the book exists
        let mut book_path = get_data_books_path();
        book_path.push(format!("{}.json", book_name));
        if !std::fs::metadata(&book_path).is_ok() {
            println!(
                "{}{}",
                "Logbook does not exist: ".bright_red(),
                book_name.bright_red()
            );
            return;
        }

        // Load the book
        let book: Logbook = autojson::structify(&book_path).expect("Could not deserialize logbook");

        // Search the contents
        for entry in book.entries.iter() {
            if callsign_re.is_match(entry.callsign.to_uppercase().as_str()) {
                // Build the table
                let table = vec![
                    vec!["Callsign", &entry.callsign.bright_blue().to_string()],
                    vec![
                        "Name",
                        &entry
                            .name
                            .as_ref()
                            .unwrap_or(&String::new())
                            .bright_blue()
                            .to_string(),
                    ],
                    vec!["Date", &entry.time.to_string().bright_blue().to_string()],
                    vec![
                        "Frequency",
                        &entry.frequency_khz.to_string().bright_blue().to_string(),
                    ],
                    vec!["Mode", &entry.mode.bright_blue().to_string()],
                    vec![
                        "RST",
                        &format!(
                            "IN: {} OUT: {}",
                            entry.recv_rst.as_ref().unwrap_or(&"???".to_string()).bright_blue(),
                            entry.sent_rst.as_ref().unwrap_or(&"???".to_string()).bright_blue()
                        ),
                    ],
                    vec![
                        "Grid",
                        &entry
                            .grid
                            .as_ref()
                            .unwrap_or(&String::new())
                            .bright_blue()
                            .to_string(),
                    ],
                    vec![
                        "Notes",
                        &entry
                            .notes
                            .as_ref()
                            .unwrap_or(&String::new())
                            .bright_blue()
                            .to_string(),
                    ],
                ]
                .table();

                // Print the table
                cli_table::print_stdout(table).unwrap();
            }
        }
    }
}
