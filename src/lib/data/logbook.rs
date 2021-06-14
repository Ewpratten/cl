use serde::{Deserialize, Serialize};

use crate::lib::{ensure_storage_location_exists, get_data_default_book_path};

use super::logentry::LogEntry;

/// Defines a single logbook
#[derive(Debug, Serialize, Deserialize)]
pub struct Logbook {
    /// Book name
    pub name: String,

    /// Possible gridsquare attached to this book
    pub grid: Option<String>,

    /// Possible callsign attached to this book
    pub callsign: Option<String>,

    /// Possible description attached to this book
    pub description: Option<String>,

    /// All entries in the book
    pub entries: Vec<LogEntry>,
}

pub fn book_name_or_default(name: Option<&str>) -> String {
    // Ensure we have our directories
    ensure_storage_location_exists();

    // Load the correct logbook
    match name {
        Some(name) => name.to_string(),
        None => std::fs::read_to_string(get_data_default_book_path())
            .expect("No default logbook to read from"),
    }
}
