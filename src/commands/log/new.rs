use colored::Colorize;

use crate::lib::{data::{logbook::{Logbook, book_name_or_default}, logentry::LogEntry}, ensure_storage_location_exists, get_data_books_path, get_data_default_book_path};

pub fn exec_new_log(
    callsign: &str,
    frequency: f32,
    mode: &str,
    logbook: Option<&str>,
    date: Option<&str>,
    time: Option<&str>,
    grid: Option<&str>,
    name: Option<&str>,
    notes: Option<&str>,
) {
    // Ensure we have our directories
    ensure_storage_location_exists();

    // Load the correct logbook
    let book_name = book_name_or_default(logbook);

    // Check if the book exists
    let mut book_path = get_data_books_path();
    book_path.push(format!("{}.json", book_name));
    if !std::fs::metadata(&book_path).is_ok() {
        println!("{}", "Logbook does not exist!".bright_red());
        return;
    }

    // Load the book
    let mut book: Logbook = autojson::structify(&book_path).expect("Could not deserialize logbook");

    // Create a log entry
    let entry = LogEntry::new(callsign, frequency, mode, date, time, grid, name, notes)
        .expect("Invalid input data");

    // Add the entry to the book
    book.entries.push(entry);

    // Write the book
    autojson::jsonify(&book, &book_path).expect("Failed to serialize logbook");

    // Log the entry id
    println!(
        "Saved log with id: {}",
        format!("{}:{}", book_name, (book.entries.len() - 1)).bright_blue()
    );
}
