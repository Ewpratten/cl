use colored::Colorize;

use crate::lib::{data::logbook::Logbook, ensure_storage_location_exists, get_data_books_path, get_data_default_book_path};

pub fn exec_new_book(
    name: &str,
    grid: Option<&str>,
    callsign: Option<&str>,
    description: Option<&str>,
    default: bool,
    import: Option<&str>,
) {
    // Ensure we have our directories
    ensure_storage_location_exists();

    // If the book already exists, error out
    let mut book_path = get_data_books_path();
    book_path.push(format!("{}.json", name));
    if std::fs::metadata(&book_path).is_ok() {
        println!("{}", "Logbook already exists!".bright_red());
        println!("Did you mean to use the \"edit\" command?");
        return;
    }

    // Create a logbook
    let mut book = Logbook {
        name: name.to_string(),
        grid: grid.map(Into::into),
        callsign: callsign.map(Into::into),
        description: description.map(Into::into),
        entries: Vec::new(),
    };

    // Handle import
    if let Some(import_path) = import {
        panic!("Importing ADIF files is not yet supported");
    }

    // Save the logbook
    println!("{:?}", book_path);
    autojson::jsonify(&book, &book_path).expect("Failed to serialize logbook");

    // Handle saving this as the default
    if default {
        std::fs::write(get_data_default_book_path(), name)
            .expect("Failed to set book as default");
        println!("Set book as default");
    }

    // Log success
    println!("New logbook created");
}
