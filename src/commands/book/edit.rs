use colored::Colorize;

use crate::lib::{data::logbook::Logbook, ensure_storage_location_exists, get_data_books_path, get_data_default_book_path};

pub fn exec_edit_book(
    name: &str,
    grid: Option<&str>,
    callsign: Option<&str>,
    description: Option<&str>,
    default: bool,
) {
    // Ensure we have our directories
    ensure_storage_location_exists();

    // Check if the book exists
    let mut book_path = get_data_books_path();
    book_path.push(format!("{}.json", name));
    if !std::fs::metadata(&book_path).is_ok() {
        println!("{}", "Logbook does not exist!".bright_red());
        println!("Did you mean to use the \"new\" command?");
        return;
    }

    // Load the book
    let mut book: Logbook = autojson::structify(&book_path).expect("Could not deserialize logbook");

    // Handle overriding data
    if let Some(grid) = grid {
        book.grid = Some(grid.to_string());
    } else if let Some(callsign) = callsign {
        book.callsign = Some(callsign.to_string());
    } else if let Some(description) = description {
        book.description = Some(description.to_string());
    }

    // Save the book
    autojson::jsonify(&book, &book_path).expect("Failed to serialize logbook");

    // Handle saving this as the default
    if default {
        std::fs::write(get_data_default_book_path(), name)
            .expect("Failed to set book as default");
        println!("Set book as default");
    }

    // Log success
    println!("Logbook updated");

}
