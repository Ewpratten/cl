use serde::{Serialize, Deserialize};

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
    pub entries: Vec<LogEntry>
}